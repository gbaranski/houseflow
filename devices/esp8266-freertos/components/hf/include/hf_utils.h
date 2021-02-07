#ifndef HF_UTILS_H
#define HF_UTILS_H

#include <esp_err.h>
#include <stdbool.h>
#include <cJSON.h>
#include "hf_types.h"

#define UTILS_TAG "utils"

esp_err_t parse_mqtt_payload(char *sig, char *msg, char *src, int src_len);
esp_err_t parse_device_request_body(DeviceRequestBody *dst, char *msg);
cJSON *stringify_device_response(const DeviceResponseBody *src);

#endif // ESP8266_FREERTOS_HF_MQTT_H
