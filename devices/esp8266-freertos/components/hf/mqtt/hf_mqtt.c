//
// Created by gbaranski on 20/12/2020.
//

#include "hf_mqtt.h"

#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>

#include "hf_crypto.h"
#include "hf_utils.h"
#include "hf_io.h"
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

#define LED_PIN 5

static esp_err_t on_data(esp_mqtt_event_handle_t event)
{
  char sig[ED25519_BASE64_SIGNATURE_BYTES + 1];
  // length of msg = signature_len - len('.')
  char msg[event->data_len - ED25519_BASE64_SIGNATURE_BYTES];
  esp_err_t err = parse_mqtt_payload(sig, msg, event->data, event->data_len);
  if (err != ESP_OK)
  {
    ESP_LOGE(MQTT_TAG, "err parsing payload %d\n", err);
    return err;
  }
  DeviceRequest req;
  err = parse_device_request(&req, msg);
  if (err != ESP_OK)
  {
    ESP_LOGE(MQTT_TAG, "fail parse device_request %d\n", err);
    return err;
  }

  DeviceResponse device_res = io_handle_command(req.command, &req);

  cJSON *device_res_json = stringify_device_response(&device_res);
  const char *device_res_str = cJSON_PrintUnformatted(device_res_json);

  char res_payload[ED25519_BASE64_SIGNATURE_BYTES + strlen(device_res_str)];
  err = crypto_sign_response_combined(res_payload, &device_res, device_res_str);
  if (err != ESP_OK)
  {
    ESP_LOGE(MQTT_TAG, "fail sign response %d\n", err);
    return err;
  }

  printf("res_str: %s\n", device_res_str);
  esp_mqtt_client_publish(event->client, CONFIG_DEVICE_ID "/command/response", res_payload, strlen(res_payload), 0, 0);

  return ESP_OK;
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

void mqtt_init(void)
{
  unsigned char password[ED25519_BASE64_SIGNATURE_BYTES];
  int err = crypto_generate_password(password);
  if (err != 0)
  {
    ESP_LOGE(MQTT_TAG, "fail gen password %d", err);
    return;
  }

  const esp_mqtt_client_config_t mqtt_cfg = {
      .uri = CONFIG_BROKER_URL,
      .client_id = CONFIG_DEVICE_ID,
      .username = CONFIG_DEVICE_PUBLIC_KEY,
      .password = (const char *)&password,
  };
  ESP_LOGI(MQTT_TAG, "[APP] Free memory: %d bytes", esp_get_free_heap_size());
  esp_mqtt_client_handle_t client = esp_mqtt_client_init(&mqtt_cfg);
  esp_mqtt_client_register_event(client, ESP_EVENT_ANY_ID, mqtt_event_handler,
                                 client);
  esp_mqtt_client_start(client);
}