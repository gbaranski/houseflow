#include "hf_mqtt.h"

#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>

#include "esp_err.h"
#include <esp_log.h>
#include <esp_system.h>
#include <esp_tls.h>
#include "hf_crypto.h"
#include <mbedtls/base64.h>
#include <mqtt_client.h>
#include <nvs_flash.h>
#include <sodium.h>
#include <driver/gpio.h>
#include <cJSON.h>

#include "hf_crypto.h"
#include "hf_device.h"

#define COMMAND_TOPIC_REQUEST CONFIG_DEVICE_ID "/command/request"
#define COMMAND_TOPIC_RESPONSE CONFIG_DEVICE_ID "/command/response"

#define STATE_TOPIC_REQUEST CONFIG_DEVICE_ID "/state/request"
#define STATE_TOPIC_RESPONSE CONFIG_DEVICE_ID "/state/response"

static esp_err_t on_command(esp_mqtt_event_handle_t event)
{
  uint16_t body_len = event->data_len - ED25519_SIGNATURE_BYTES - REQUEST_ID_SIZE;
  char sig[ED25519_SIGNATURE_BYTES];
  uint8_t requestID[REQUEST_ID_SIZE];
  char body[body_len + 1];

  memcpy(sig, event->data, ED25519_SIGNATURE_BYTES);
  memcpy(requestID, &event->data[ED25519_SIGNATURE_BYTES], REQUEST_ID_SIZE);
  memcpy(body, &event->data[ED25519_SIGNATURE_BYTES + REQUEST_ID_SIZE], body_len);
  body[body_len] = '\0';

  ESP_LOGI(MQTT_TAG, "req_body: %.*s", body_len, body);

  bool valid = crypto_verify_server_payload(sig, requestID, body, body_len);
  if ( !valid ) {
    ESP_LOGE(MQTT_TAG, "server sent invalid signature");
    return ESP_ERR_INVALID_RESPONSE;
  }
  ESP_LOGI(MQTT_TAG, "signature verified");
  
  cJSON* req_json = cJSON_Parse(body);
  if (req_json == NULL) {
    const char *error_ptr = cJSON_GetErrorPtr();
    ESP_LOGE(DEVICE_TAG, "fail parsing json %s", error_ptr);
    return ESP_ERR_INVALID_RESPONSE;
  }
  cJSON* command_item = cJSON_GetObjectItemCaseSensitive(req_json, "command");
  cJSON* params_item = cJSON_GetObjectItemCaseSensitive(req_json, "params");
  if (!cJSON_IsString(command_item)) {
    ESP_LOGE(MQTT_TAG, "'command' in JSON is invalid, expected type string");
    return ESP_ERR_INVALID_RESPONSE;
  }
  if (command_item->valuestring == NULL) {
    ESP_LOGE(MQTT_TAG, "unexpected 'command' NULL");
    return ESP_ERR_INVALID_RESPONSE;
  }
  if (!cJSON_IsObject(params_item)) {
    ESP_LOGE(MQTT_TAG, "'params' in JSON is invalid, expected type object");
    return ESP_ERR_INVALID_RESPONSE;
  }


  cJSON* res_json = device_execute(command_item->string, params_item);
  const char* res_body = cJSON_PrintUnformatted(res_json);

  size_t res_body_len = strlen(res_body);
  ESP_LOGI(MQTT_TAG, "res_body: %s\n", res_body);

  char res_payload[ED25519_SIGNATURE_BYTES + REQUEST_ID_SIZE + res_body_len];
  // Add requestID to res payload
  memcpy(&res_payload[ED25519_SIGNATURE_BYTES], requestID, REQUEST_ID_SIZE);
  // Add JSON to res payload
  memcpy(&res_payload[ED25519_SIGNATURE_BYTES + REQUEST_ID_SIZE], res_body, res_body_len);

  // Sign the above data(requestID, JSON)
  // Destination is the beginning of the payload
  esp_err_t err = crypto_sign_payload((unsigned char *)res_payload, &res_payload[ED25519_SIGNATURE_BYTES], REQUEST_ID_SIZE + res_body_len);
  if (err != ESP_OK)
  {
    ESP_LOGE(MQTT_TAG, "fail sign payload with base64 encoding code: %d", err);
    return err;
  }

  esp_mqtt_client_publish(
      event->client,
      COMMAND_TOPIC_RESPONSE,
      res_payload,
      ED25519_SIGNATURE_BYTES + REQUEST_ID_SIZE + res_body_len,
      0,
      0);

  return ESP_OK;
}

static esp_err_t on_fetch_state(esp_mqtt_event_handle_t event)
{
  char sig[ED25519_SIGNATURE_BYTES];
  uint8_t requestID[REQUEST_ID_SIZE];

  memcpy(sig, event->data, ED25519_SIGNATURE_BYTES);
  memcpy(requestID, &event->data[ED25519_SIGNATURE_BYTES], REQUEST_ID_SIZE);

  bool valid = crypto_verify_server_payload(sig, requestID, NULL, 0);
  if ( !valid ) {
    ESP_LOGE(MQTT_TAG, "server sent invalid signature");
    return ESP_ERR_INVALID_RESPONSE;
  }
  ESP_LOGI(MQTT_TAG, "signature verified");

  cJSON *res_body_json = device_query();
  const char *res_body = cJSON_PrintUnformatted(res_body_json);
  size_t res_body_len = strlen(res_body);
  ESP_LOGI(MQTT_TAG, "res_body: %s\n", res_body);

  char res_payload[ED25519_SIGNATURE_BYTES + REQUEST_ID_SIZE + res_body_len];
  // Add requestID to res payload
  memcpy(&res_payload[ED25519_SIGNATURE_BYTES], requestID, REQUEST_ID_SIZE);
  // Add JSON to res payload
  memcpy(&res_payload[ED25519_SIGNATURE_BYTES + REQUEST_ID_SIZE], res_body, res_body_len);

  // Sign the above data(requestID, JSON)
  // Destination is the beginning of the payload
  esp_err_t err = crypto_sign_payload(
      (unsigned char *)res_payload, 
      &res_payload[ED25519_SIGNATURE_BYTES], 
      REQUEST_ID_SIZE + res_body_len);

  if (err != ESP_OK)
  {
    ESP_LOGE(MQTT_TAG, "fail sign payload with base64 encoding code: %d", err);
    return err;
  }

  esp_mqtt_client_publish(
      event->client,
      STATE_TOPIC_RESPONSE,
      res_payload,
      ED25519_SIGNATURE_BYTES + REQUEST_ID_SIZE + res_body_len,
      0,
      0);

  return ESP_OK;
}

static esp_err_t on_data(esp_mqtt_event_handle_t event)
{
  ESP_LOGI(MQTT_TAG, "Received data on %.*s topic", event->topic_len, event->topic);
  if (memcmp(event->topic, COMMAND_TOPIC_REQUEST, event->topic_len) == 0)
  {
    ESP_LOGI(MQTT_TAG, "Executing onCommand");
    return on_command(event);
  }
  else if (memcmp(event->topic, STATE_TOPIC_REQUEST, event->topic_len) == 0)
  {
    ESP_LOGI(MQTT_TAG, "Executing onFetchState");
    return on_fetch_state(event);
  }
  else
  {
    ESP_LOGE(MQTT_TAG, "Unrecognized topic: %s", event->topic);
    return ESP_ERR_INVALID_RESPONSE;
  }
}

static esp_err_t mqtt_event_handler_cb(esp_mqtt_event_handle_t event)
{
  esp_mqtt_client_handle_t client = event->client;
  int msg_id;
  // your_context_t *context = event->context;
  switch (event->event_id)
  {
  case MQTT_EVENT_CONNECTED:
    ESP_LOGI(MQTT_TAG, "MQTT_EVENT_CONNECTED");
    msg_id = esp_mqtt_client_subscribe(
        client, COMMAND_TOPIC_REQUEST, 0);
    ESP_LOGI(MQTT_TAG, "subscribed %s, msg_id=%d", COMMAND_TOPIC_REQUEST, msg_id);

    msg_id = esp_mqtt_client_subscribe(
        client, STATE_TOPIC_REQUEST, 0);
    ESP_LOGI(MQTT_TAG, "subscribed %s, msg_id=%d", STATE_TOPIC_REQUEST, msg_id);

    break;
  case MQTT_EVENT_DISCONNECTED:
    ESP_LOGI(MQTT_TAG, "MQTT_EVENT_DISCONNECTED");
    break;

  case MQTT_EVENT_SUBSCRIBED:
    ESP_LOGI(MQTT_TAG, "MQTT_EVENT_SUBSCRIBED, msg_id=%d", event->msg_id);
    //            msg_id = esp_mqtt_client_publish(client, "/topic/qos0",
    //            "data", 0, 0, 0); ESP_LOGI(MQTT_TAG, "sent publish
    //            successful, msg_id=%d", msg_id);
    break;
  case MQTT_EVENT_UNSUBSCRIBED:
    ESP_LOGI(MQTT_TAG, "MQTT_EVENT_UNSUBSCRIBED, msg_id=%d", event->msg_id);
    break;
  case MQTT_EVENT_PUBLISHED:
    ESP_LOGI(MQTT_TAG, "MQTT_EVENT_PUBLISHED, msg_id=%d", event->msg_id);
    break;
  case MQTT_EVENT_DATA:
  {
    ESP_LOGI(MQTT_TAG, "MQTT_EVENT_DATA");
    on_data(event);
    break;
  }
  case MQTT_EVENT_ERROR:
    ESP_LOGI(MQTT_TAG, "MQTT_EVENT_ERROR");
    if (event->error_handle->error_type == MQTT_ERROR_TYPE_ESP_TLS)
    {
      ESP_LOGI(MQTT_TAG, "Last error code reported from esp-tls: 0x%x",
               event->error_handle->esp_tls_last_esp_err);
      ESP_LOGI(MQTT_TAG, "Last tls stack error number: 0x%x",
               event->error_handle->esp_tls_stack_err);
    }
    else if (event->error_handle->error_type ==
             MQTT_ERROR_TYPE_CONNECTION_REFUSED)
    {
      ESP_LOGI(MQTT_TAG, "Connection refused error: 0x%x",
               event->error_handle->connect_return_code);
    }
    else
    {
      ESP_LOGW(MQTT_TAG, "Unknown error type: 0x%x",
               event->error_handle->error_type);
    }
    break;
  default:
    ESP_LOGI(MQTT_TAG, "Other event id:%d", event->event_id);
    break;
  }
  return ESP_OK;
}

static void mqtt_event_handler(void *handler_args, esp_event_base_t base,
                               int32_t event_id, void *event_data)
{
  ESP_LOGD(MQTT_TAG, "Event dispatched from event loop base=%s, event_id=%d",
           base, event_id);
  mqtt_event_handler_cb(event_data);
}

esp_err_t mqtt_init(void)
{
  unsigned char password[PASSWORD_BYTES];
  int err = crypto_generate_password(password);
  if (err != 0)
  {
    ESP_LOGE(MQTT_TAG, "fail gen password %d", err);
    return err;
  }

  unsigned char password_base64[PASSWORD_BASE64_BYTES + 1];
  err = crypto_encode_password(password_base64, password);
  if (err != 0) {
    ESP_LOGE(MQTT_TAG, "fail encode password %d", err);
    return err;
  }

  const esp_mqtt_client_config_t mqtt_cfg = {
      .uri = CONFIG_BROKER_URL,
      .client_id = CONFIG_DEVICE_ID,
      .username = CONFIG_DEVICE_PUBLIC_KEY,
      .password = (const char *)password_base64,
  };
  ESP_LOGI(MQTT_TAG, "[APP] Free memory: %d bytes", esp_get_free_heap_size());
  esp_mqtt_client_handle_t client = esp_mqtt_client_init(&mqtt_cfg);
  ESP_ERROR_CHECK(esp_mqtt_client_register_event(client, ESP_EVENT_ANY_ID, mqtt_event_handler,
                                 client));
  ESP_ERROR_CHECK(esp_mqtt_client_start(client));

  return ESP_OK;
}
