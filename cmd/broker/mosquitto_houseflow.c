#include <stdio.h>
#include <string.h>

#include <libpq-fe.h>
#include <mosquitto_broker.h>
#include <mosquitto_plugin.h>
#include <mosquitto.h>
#include <mqtt_protocol.h>
#include <sodium.h>


#define ED25519_PKEY_BASE64_BYTES 44
#define ED25519_PKEY_BYTES 32

static mosquitto_plugin_id_t *mosq_pid = NULL;
static PGconn *pgconn = NULL;

static int auth_cb(int event, void *event_data, void *userdata)
{
  printf("basic_auth_callback()\n");
	struct mosquitto_evt_basic_auth *ed = event_data;
  if(strlen(ed->username) != ED25519_PKEY_BASE64_BYTES) {
    printf("invalid username len: %lu\n", strlen(ed->username));
    return MOSQ_ERR_AUTH;
  }

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
