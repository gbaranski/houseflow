//
// Created by gbaranski on 20/12/2020.
//

#include "hf_mqtt.h"

#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>

#include "hf_crypto.h"
#include "esp_log.h"
#include "esp_system.h"
#include "esp_tls.h"
#include "hf_crypto.h"
#include "mbedtls/base64.h"
#include "mqtt_client.h"
#include "nvs_flash.h"
#include "sodium.h"
#include "driver/gpio.h"
#include "cJSON.h"

#define LED_PIN 5

// ED25519_BASE64_SIGNATURE_LENGTH + strlen('.') + strlen("{}") + \0
#define MQTT_MIN_PAYLOAD_SIZE ED25519_BASE64_SIGNATURE_BYTES + 1 + 2 + 1

struct Keypair kp;

typedef struct {
  bool on;
} device_state;

typedef struct {
  const char* correlation_data;
  const char* command;
  device_state state;
} device_request;

typedef struct {
  const char* correlation_data;
  // SUCCESS | ERROR
  const char* status;
  // Present only if status == ERROR
  const char* error;
  device_state state;
} device_response;

static esp_err_t parse_payload(char *sig, char *msg, char *src, int src_len)
{
  if (src_len < MQTT_MIN_PAYLOAD_SIZE)
  {
    ESP_LOGE(MQTT_TAG, "payload is too small");
    return ESP_ERR_INVALID_RESPONSE;
  }

  memcpy(sig, src, ED25519_BASE64_SIGNATURE_BYTES);
  sig[ED25519_BASE64_SIGNATURE_BYTES] = '\0';

  printf("msglen: %d\n", src_len - ED25519_BASE64_SIGNATURE_BYTES);

  memcpy(msg, &(src[ED25519_BASE64_SIGNATURE_BYTES + 1]), src_len - ED25519_BASE64_SIGNATURE_BYTES);
  msg[strlen(msg) - 1] = '\0';
  return ESP_OK;
}

static esp_err_t parse_device_state(device_state *dst, cJSON *json) {
  cJSON *onItem = cJSON_GetObjectItemCaseSensitive(json, "on");
  if (!cJSON_IsBool(onItem))
  {
    ESP_LOGE(MQTT_TAG, "state.on is not boolean");
    return ESP_ERR_INVALID_RESPONSE;
  }

  if (cJSON_IsTrue(onItem))
    dst->on = true;
  else if (cJSON_IsFalse(onItem))
    dst->on = false;
  else
  {
    ESP_LOGE(MQTT_TAG, "state.on != true && state.on != false");
    return ESP_ERR_INVALID_RESPONSE;
  }

  return ESP_OK;
}

static esp_err_t parse_device_request(device_request *dst, char *msg)
{
  cJSON *json = cJSON_Parse(msg);

  if (json == NULL)
  {
    const char *error_ptr = cJSON_GetErrorPtr();
    ESP_LOGE("fail parse json %s\n", error_ptr);

    return ESP_ERR_INVALID_RESPONSE;
  }

  cJSON *correlationDataItem = cJSON_GetObjectItemCaseSensitive(json, "correlationData");
  cJSON *commandItem = cJSON_GetObjectItemCaseSensitive(json, "command");
  if (!cJSON_IsString(correlationDataItem) || (correlationDataItem->valuestring == NULL)) {
    ESP_LOGE(MQTT_TAG, "correlationData field is invalid string");
    return ESP_ERR_INVALID_RESPONSE;
  }
  if (!cJSON_IsString(commandItem) || (commandItem->valuestring == NULL)) {
    ESP_LOGE(MQTT_TAG, "command field is invalid string");
    return ESP_ERR_INVALID_RESPONSE;
  }

  dst->command = commandItem->valuestring;
  dst->correlation_data = correlationDataItem->valuestring;

  cJSON *stateItem = cJSON_GetObjectItemCaseSensitive(json, "state");
  if (!cJSON_IsObject(stateItem) || (stateItem == NULL))
  {
    ESP_LOGE(MQTT_TAG, "stateItem is not object or stateItem == NULL");
    return ESP_ERR_INVALID_RESPONSE;
  }
  return parse_device_state(&(dst->state), stateItem);
}

static cJSON* stringify_device_response(const device_response *src) {
  // Create "state" field
  cJSON *state = cJSON_CreateObject();
  // if device responds, it means its online
  cJSON_AddBoolToObject(state, "online", true);
  cJSON_AddBoolToObject(state, "on", src->state.on);

  cJSON *root;
  root = cJSON_CreateObject();
  cJSON_AddStringToObject(root, "correlationData", src->correlation_data);
  cJSON_AddStringToObject(root, "status", src->status);
  if (src->error != NULL) {
    cJSON_AddStringToObject(root, "error", src->error);
  }
  cJSON_AddItemToObject(root, "state", state);

  return root;
}

static esp_err_t mqtt_event_handler_cb(esp_mqtt_event_handle_t event) {
  esp_mqtt_client_handle_t client = event->client;
  int msg_id;
  // your_context_t *context = event->context;
  switch (event->event_id) {
    case MQTT_EVENT_CONNECTED:
      ESP_LOGI(MQTT_TAG, "MQTT_EVENT_CONNECTED");
      msg_id = esp_mqtt_client_subscribe(
          client, CONFIG_DEVICE_ID "/command/request", 0);
      ESP_LOGI(MQTT_TAG, "sent subscribe successful, msg_id=%d", msg_id);
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
    case MQTT_EVENT_DATA: {

      ESP_LOGI(MQTT_TAG, "MQTT_EVENT_DATA");

      char sig[ED25519_BASE64_SIGNATURE_BYTES + 1];

      // length of msg = signature_len - len('.')
      char msg[event->data_len - ED25519_BASE64_SIGNATURE_BYTES];
      esp_err_t err = parse_payload(sig, msg, event->data, event->data_len);
      if (err != ESP_OK) {
        ESP_LOGI(MQTT_TAG, "err parsing payload %d\n", err);
        return err;
      }
      device_request req;
      err = parse_device_request(&req, msg);
      ESP_LOGI(MQTT_TAG, "correlationData: %s", req.correlation_data);
      ESP_LOGI(MQTT_TAG, "command: %s", req.command);
      ESP_LOGI(MQTT_TAG, "state.on: %d", req.state.on);

      printf("signature: %s\n", sig);
      printf("msg: %s\n", msg);

      gpio_set_level(LED_PIN, req.state.on);

      const device_response res = {
        .correlation_data = req.correlation_data,
        .status = "SUCCESS",
        .error = NULL,
        .state = req.state,
      };
      cJSON *resJSON = stringify_device_response(&res);
      const char* res_str = cJSON_PrintUnformatted(resJSON);
      printf("stringified: %s\n", res_str);

      // fix this buffer later, not sure if we need that big buffer for just signature
      unsigned char res_sig[ED25519_SIGNATURE_BYTES];
      int sign_err = crypto_sign_detached(res_sig, NULL, (const unsigned char*)res_str, strlen(res_str), kp.skey);
      if (sign_err != 0) {
        printf("fail sign code: %d\n", sign_err);
        return ESP_ERR_INVALID_RESPONSE;
      }
      unsigned char res_sig_encoded[ED25519_BASE64_SIGNATURE_BYTES ];
      crypto_err_t crypto_err = encode_signature(res_sig, res_sig_encoded);
      if (crypto_err != CRYPTO_ERR_OK) {
        printf("fail encode response sig %d\n", crypto_err);
        // ESP_LOGE("fail encode signature %d",(int)crypto_err);
        return ESP_ERR_INVALID_RESPONSE;
      }
      printf("response signature: %s\n", res_sig_encoded);
      
      char full_response[strlen(res_str) + ED25519_SIGNATURE_BYTES];
      strcpy(full_response, (const char*)res_sig_encoded);
      strcat(full_response, ".");
      strcat(full_response, res_str);

      printf("full response: %s\n", full_response);
      esp_mqtt_client_publish(client, CONFIG_DEVICE_ID "/command/response", full_response, strlen(full_response), 0, 0);
      break;
    }
    case MQTT_EVENT_ERROR:
      ESP_LOGI(MQTT_TAG, "MQTT_EVENT_ERROR");
      if (event->error_handle->error_type == MQTT_ERROR_TYPE_ESP_TLS) {
        ESP_LOGI(MQTT_TAG, "Last error code reported from esp-tls: 0x%x",
                 event->error_handle->esp_tls_last_esp_err);
        ESP_LOGI(MQTT_TAG, "Last tls stack error number: 0x%x",
                 event->error_handle->esp_tls_stack_err);
      } else if (event->error_handle->error_type ==
                 MQTT_ERROR_TYPE_CONNECTION_REFUSED) {
        ESP_LOGI(MQTT_TAG, "Connection refused error: 0x%x",
                 event->error_handle->connect_return_code);
      } else {
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
                               int32_t event_id, void *event_data) {
  ESP_LOGD(MQTT_TAG, "Event dispatched from event loop base=%s, event_id=%d",
           base, event_id);
  mqtt_event_handler_cb(event_data);
}

void mqtt_connect() {
  gpio_set_direction(LED_PIN, GPIO_MODE_OUTPUT);
  
  crypto_err_t err = get_public_key(&kp);
  if (err != CRYPTO_ERR_OK) {
    ESP_LOGE(MQTT_TAG, "fail read public_key err: %d", err);
    return;
  }
  err = get_private_key(&kp);
  if (err != CRYPTO_ERR_OK) {
    ESP_LOGE(MQTT_TAG, "fail read private_key err: %d", err);
    return;
  }

  unsigned char password[ED25519_SIGNATURE_BYTES];
  err = crypto_sign_ed25519_detached(password, NULL, kp.pkey, ED25519_PKEY_BYTES, kp.skey);
  if (err != 0) {
    ESP_LOGE(MQTT_TAG, "fail gen password err: %d", err);
    return;
  }
  unsigned char encoded_password[ED25519_BASE64_SIGNATURE_BYTES];
  err = encode_signature(password, encoded_password);
  if (err != CRYPTO_ERR_OK) {
    ESP_LOGE(MQTT_TAG, "fail encode password err: %d", err);
    return;
  }

  const esp_mqtt_client_config_t mqtt_cfg = {
      .uri = CONFIG_BROKER_URL,
      .client_id = CONFIG_DEVICE_ID,
      .username = CONFIG_DEVICE_PUBLIC_KEY,
      .password = (const char *)&encoded_password,
  };
  ESP_LOGI(MQTT_TAG, "[APP] Free memory: %d bytes", esp_get_free_heap_size());
  esp_mqtt_client_handle_t client = esp_mqtt_client_init(&mqtt_cfg);
  esp_mqtt_client_register_event(client, ESP_EVENT_ANY_ID, mqtt_event_handler,
                                 client);
  esp_mqtt_client_start(client);
}