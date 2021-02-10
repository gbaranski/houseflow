#include <netinet/in.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <time.h>
#include <arpa/inet.h>

#include <libpq-fe.h>
#include <mosquitto_broker.h>
#include <mosquitto_plugin.h>
#include <mosquitto.h>
#include <mqtt_protocol.h>
#include <sodium.h>

static const unsigned int SIG_BYTES = 64;
static const unsigned int SIG_BASE64_BYTES = ((4 * SIG_BYTES / 3) + 3) & ~3;

static const unsigned int PKEY_BYTES = 32;
static const unsigned int PKEY_BASE64_BYTES = ((4 * PKEY_BYTES / 3) + 3) & ~3;

static const unsigned int PASSWORD_BYTES = SIG_BYTES + sizeof(uint32_t);
static const unsigned int PASSWORD_BASE64_BYTES = ((4 * PASSWORD_BYTES / 3) + 3) & ~3;

static const unsigned int MAX_TIMESTAMP_DIFFERENCE = 3600;


static mosquitto_plugin_id_t *mosq_pid = NULL;
static PGconn *pgconn = NULL;

static int decode_signature( unsigned char* dst, const char *ptr ) {
  size_t sig_size;
  int err = sodium_base642bin(
      dst, SIG_BYTES,  // OUTPUT
      ptr, SIG_BASE64_BYTES, // INPUT
      "", &sig_size, // SOMETHING
      NULL, sodium_base64_VARIANT_ORIGINAL
      );
  if ( err != 0 )  {
    printf("fail decoding signature, err: %d\n", err);
    return 1;
  }
  if (sig_size != SIG_BYTES) {
    printf("invalid signature size: %zu\n", sig_size);
    return 1;
  }

  return 0;
}

static int decode_password( unsigned char* sig, uint8_t* timestamp, const char* const src ) {
  unsigned char pass[PASSWORD_BYTES];
  size_t pass_size;
  int err = sodium_base642bin(
      pass, PASSWORD_BYTES,
      src, PASSWORD_BASE64_BYTES,
      "", &pass_size,
      NULL, sodium_base64_VARIANT_ORIGINAL
      );
  if (err != 0) {
    printf("fail decoding password, err: %d\n", err);
    return 1;
  }
  if (pass_size != PASSWORD_BYTES) {
    printf("invalid signature size: %zu\n", pass_size);
    return 1;
  }
  memcpy(sig, pass, SIG_BYTES);
  memcpy(timestamp, &pass[SIG_BYTES], sizeof(uint32_t));
  

  return 0;
}

static int verify_password( 
    const unsigned char* const sig, 
    const uint8_t* ts, 
    const unsigned char* const pkey 
) 
{
  const uint32_t timestamp = 
    ts[3]         |
    (ts[2] << 8)  |
    (ts[1] << 16) |
    (ts[0] << 24);

  const int now = time(NULL);
  if ( timestamp > now ) {
    printf( "signature is not valid yet, too early by: %d\n", timestamp - now);
    return 1;
  }
  if ( ( now - timestamp ) > MAX_TIMESTAMP_DIFFERENCE ) {
    printf( "signature has expired by %d\n", now - timestamp - MAX_TIMESTAMP_DIFFERENCE);
    return 1;
  }


  int err = crypto_sign_verify_detached( sig, ts, sizeof( uint32_t ), pkey );
  if ( err != 0 ) {
    printf( "invalid signature\n" );
    return 1;
  }

  return 0;
}


static int auth_cb( int event, void *event_data, void *userdata )
{
	struct mosquitto_evt_basic_auth *ed = event_data;
  if(strlen(ed->username) != PKEY_BASE64_BYTES) {
    printf("invalid username len: %lu\n", strlen(ed->username));
    return MOSQ_ERR_AUTH;
  }
  unsigned char pkey[PKEY_BYTES];

  size_t pkey_size;
  int err = sodium_base642bin(
      pkey, PKEY_BYTES,  // OUTPUT
      ed->username, PKEY_BASE64_BYTES, // INPUT
      "", &pkey_size, // SOMETHING
      NULL, sodium_base64_VARIANT_ORIGINAL
      );
  if ( err != 0 )  {
    printf("fail decoding pkey, err: %d\n", err);
    return MOSQ_ERR_AUTH;
  }
  if ( pkey_size != PKEY_BYTES ) {
    printf("invalid decoded pkey size: %zu\n", pkey_size);
    return MOSQ_ERR_AUTH;
  }

  unsigned char sig[SIG_BYTES];
  uint8_t ts[sizeof(uint32_t)];
  err = decode_password(sig, ts, ed->password);
  if ( err != 0 ) {
    return MOSQ_ERR_AUTH;
  }

  err = verify_password( sig, ts, pkey );
  if ( err != 0 ) {
    return MOSQ_ERR_AUTH;
  }

  printf("valid signature\n");

	return MOSQ_ERR_SUCCESS;
}

int mosquitto_plugin_version(int supported_version_count, const int *supported_versions)
{
	for(int i = 0; i < supported_version_count; i++){
		if(supported_versions[i] == 5){
			return 5;
		}
	}
	return -1;
}

char* must_getenv(const char* key) {
  char* env = getenv(key);
  if (env == NULL) {
    printf("%s env not found\n", key);
    return NULL;
  }
  return env;
}

#define POSTGRES_MAX_ATTEMPTS 10

int init_postgres() {
  char* host = must_getenv("POSTGRES_HOST");
  char* port = getenv("POSTGRES_PORT");
  char* password = getenv("POSTGRES_PASSWORD");
  char* dbname = getenv("POSTGRES_DB");
  if (host == NULL || password == NULL || dbname == NULL || port == NULL) return 1;

  char params[1000];
  sprintf(params, "host=%s port=%s user=postgres password=%s dbname=%s", host, port, password, dbname);

  PGPing ping_response = PQPING_NO_ATTEMPT;
  for(uint retry = 0; ping_response != PQPING_OK; retry++) {
    printf( "checking if postgres is active\n" );
    if (retry > POSTGRES_MAX_ATTEMPTS) {
      printf( "max attempts exceeded\n" );
      return 1;
    }

    ping_response = PQping(params);
    switch (ping_response) {
      case PQPING_OK:
        printf("postgres active, connecting\n");
        break;
      case PQPING_REJECT:
      case PQPING_NO_RESPONSE:
      case PQPING_NO_ATTEMPT:
        printf( "waiting postgres status: %x, retry: %d, left: %d\n", ping_response, retry, POSTGRES_MAX_ATTEMPTS-retry );
        sleep( 1 );
    }
  }

  pgconn = PQconnectdb(params);
  if (PQstatus(pgconn) != CONNECTION_OK) {
    printf("Connection to database failed: %s\n", PQerrorMessage(pgconn));
    PQfinish(pgconn);
    return 1;
  }
  printf("Connected to PostgreSQL\n");

  return 0;
}

int mosquitto_plugin_init(mosquitto_plugin_id_t *identifier, void **user_data, struct mosquitto_opt *opts, int opt_count)
{
	mosq_pid = identifier;

  if(sodium_init() == -1) return 1;
  printf("Sodium initialized\n");

  if (init_postgres() != 0) {
    printf("failed initializing postgres\n");
    return 1;
  }

	return mosquitto_callback_register(mosq_pid, MOSQ_EVT_BASIC_AUTH, auth_cb, NULL, NULL);
}

int mosquitto_plugin_cleanup(void *user_data, struct mosquitto_opt *opts, int opt_count)
{
  printf("Disconnecting from PostgreSQL\n");
  PQfinish(pgconn);
	return mosquitto_callback_unregister(mosq_pid, MOSQ_EVT_BASIC_AUTH, auth_cb, NULL);
}
