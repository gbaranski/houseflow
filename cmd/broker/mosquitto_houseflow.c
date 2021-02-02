#include <stdio.h>
#include <string.h>

#include <libpq-fe.h>
#include <mosquitto_broker.h>
#include <mosquitto_plugin.h>
#include <mosquitto.h>
#include <mqtt_protocol.h>

static mosquitto_plugin_id_t *mosq_pid = NULL;
static PGconn *pgconn = NULL;

static int basic_auth_callback(int event, void *event_data, void *userdata)
{
  printf("basic_auth_callback()\n");
	struct mosquitto_evt_basic_auth *ed = event_data;
	const char *ip_address = mosquitto_client_address(ed->client);

	if(!strcmp(ip_address, "127.0.0.1")){
		/* Only allow connections from localhost */
		return MOSQ_ERR_SUCCESS;
	}else{
		return MOSQ_ERR_AUTH;
	}
}

int mosquitto_plugin_version(int supported_version_count, const int *supported_versions)
{
  printf("mosquitto_plugin_version()\n");
	int i;

	for(i=0; i<supported_version_count; i++){
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

  pgconn = PQconnectdb("user=postgres dbname=gbaranski");
  if (PQstatus(pgconn) == CONNECTION_BAD) {
    printf("Connection to database failed: %s\n", PQerrorMessage(pgconn));
    PQfinish(pgconn);
    return 1;
  }
  printf("Successfully connected to PostgreSQL status: %x\n", PQstatus(pgconn));

	return mosquitto_callback_register(mosq_pid, MOSQ_EVT_BASIC_AUTH, basic_auth_callback, NULL, NULL);
}

int mosquitto_plugin_cleanup(void *user_data, struct mosquitto_opt *opts, int opt_count)
{
  printf("mosquitto_plugin_cleanup()\n");
  printf("Disconnecting from PostgreSQL\n");
  PQfinish(pgconn);
	return mosquitto_callback_unregister(mosq_pid, MOSQ_EVT_BASIC_AUTH, basic_auth_callback, NULL);
}
