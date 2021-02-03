#include <stdio.h>
#include <string.h>

#include <libpq-fe.h>
#include <mosquitto_broker.h>
#include <mosquitto_plugin.h>
#include <mosquitto.h>
#include <mqtt_protocol.h>
#include <sodium.h>


#define SIG_BYTES 64U
#define SIG_BASE64_BYTES 88U

#define PKEY_BASE64_BYTES 44U
#define PKEY_BYTES 32U

static mosquitto_plugin_id_t *mosq_pid = NULL;
static PGconn *pgconn = NULL;

static int auth_cb(int event, void *event_data, void *userdata)
{
  printf("basic_auth_callback()\n");
	struct mosquitto_evt_basic_auth *ed = event_data;
  if(strlen(ed->username) != PKEY_BASE64_BYTES) {
    printf("invalid username len: %lu\n", strlen(ed->username));
    return MOSQ_ERR_AUTH;
  }
  printf("username: %s\n", ed->username);


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

  unsigned char sig[SIG_BASE64_BYTES];
  size_t sig_size;
  err = sodium_base642bin(
      sig, SIG_BYTES,  // OUTPUT
      ed->password, SIG_BASE64_BYTES, // INPUT
      "", &sig_size, // SOMETHING
      NULL, sodium_base64_VARIANT_ORIGINAL
      );
  if ( err != 0 )  {
    printf("fail decoding signature, err: %d\n", err);
    return MOSQ_ERR_AUTH;
  }
  if ( sig_size != SIG_BYTES ) {
    printf("invalid decoded signature size: %zu\n", sig_size);
    return MOSQ_ERR_AUTH;
  }

  err = crypto_sign_verify_detached(sig, pkey, PKEY_BYTES, pkey);
  if ( err != 0 ) {
    printf("invalid signature\n");
    return MOSQ_ERR_AUTH;
  }
  printf("valid signature\n");
	return MOSQ_ERR_SUCCESS;
}

int mosquitto_plugin_version(int supported_version_count, const int *supported_versions)
{
  printf("mosquitto_plugin_version()\n");
	for(int i = 0; i < supported_version_count; i++){
		if(supported_versions[i] == 5){
			return 5;
		}
	}
	return -1;
}

int mosquitto_plugin_init(mosquitto_plugin_id_t *identifier, void **user_data, struct mosquitto_opt *opts, int opt_count)
{
  printf("mosquitto_plugin_init()\n");
	mosq_pid = identifier;

  if(sodium_init() == -1) {
    return 1;
  }
  printf("Sodium initialized\n");

  pgconn = PQconnectdb("user=postgres dbname=gbaranski");
  if (PQstatus(pgconn) != CONNECTION_OK) {
    printf("Connection to database failed: %s\n", PQerrorMessage(pgconn));
    PQfinish(pgconn);
    return 1;
  }
  printf("Connected to PostgreSQL\n");

	return mosquitto_callback_register(mosq_pid, MOSQ_EVT_BASIC_AUTH, auth_cb, NULL, NULL);
}

int mosquitto_plugin_cleanup(void *user_data, struct mosquitto_opt *opts, int opt_count)
{
  printf("mosquitto_plugin_cleanup()\n");
  printf("Disconnecting from PostgreSQL\n");
  PQfinish(pgconn);
	return mosquitto_callback_unregister(mosq_pid, MOSQ_EVT_BASIC_AUTH, auth_cb, NULL);
}
