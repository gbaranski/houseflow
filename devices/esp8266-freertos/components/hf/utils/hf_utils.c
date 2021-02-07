#include "hf_utils.h"

#include <esp_log.h>
#include <stdbool.h>
#include <string.h>
#include <cJSON.h>
#include "hf_crypto.h"
#include "hf_types.h"

static esp_err_t parse_device_state(DeviceState *dst, cJSON *json)
{
  cJSON *onItem = cJSON_GetObjectItemCaseSensitive(json, "on");
  if (!cJSON_IsBool(onItem))
  {
    ESP_LOGE(UTILS_TAG, "state.on is not boolean");
    return ESP_ERR_INVALID_RESPONSE;
  }

  if (cJSON_IsTrue(onItem))
    dst->on = true;
  else if (cJSON_IsFalse(onItem))
    dst->on = false;
  else
  {
    ESP_LOGE(UTILS_TAG, "state.on != true && state.on != false");
    return ESP_ERR_INVALID_RESPONSE;
  }

  return ESP_OK;
}

esp_err_t parse_device_request_body(DeviceRequestBody *dst, char *msg)
{
  cJSON *json = cJSON_Parse(msg);

  if (json == NULL)
  {
    const char *error_ptr = cJSON_GetErrorPtr();
    ESP_LOGE(UTILS_TAG, "fail parse json %s\n", error_ptr);

    return ESP_ERR_INVALID_RESPONSE;
  }

  cJSON *commandItem = cJSON_GetObjectItemCaseSensitive(json, "command");
  if (!cJSON_IsString(commandItem) || (commandItem->valuestring == NULL))
  {
    ESP_LOGE(UTILS_TAG, "command field is invalid string");
    return ESP_ERR_INVALID_RESPONSE;
  }

  dst->command = commandItem->valuestring;

  cJSON *stateItem = cJSON_GetObjectItemCaseSensitive(json, "state");
  if (!cJSON_IsObject(stateItem) || (stateItem == NULL))
  {
    ESP_LOGE(UTILS_TAG, "stateItem is not object or stateItem == NULL");
    return ESP_ERR_INVALID_RESPONSE;
  }
  return parse_device_state(&(dst->state), stateItem);
}

cJSON *stringify_device_response(const DeviceResponseBody *src)
{
  // Create "state" field
  cJSON *state = cJSON_CreateObject();
  // if device responds, it means its online
  cJSON_AddBoolToObject(state, "online", true);
  cJSON_AddBoolToObject(state, "on", src->state.on);

  cJSON *root;
  root = cJSON_CreateObject();
  cJSON_AddStringToObject(root, "status", src->status);
  if (src->error != NULL)
  {
    cJSON_AddStringToObject(root, "error", src->error);
  }
  cJSON_AddItemToObject(root, "state", state);

  return root;
}
