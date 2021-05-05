#include "sdkconfig.h"

#if CONFIG_DEVICE_TYPE_LIGHT == 1

#include "hf_device.h"

#include <string.h>
#include <stdbool.h>
#include <esp_err.h>
#include <esp_log.h>
#include <driver/gpio.h>
#include <cJSON.h>


typedef struct {
  bool on;
} DeviceState;

typedef struct {
  bool on;
} ExecuteParams;

esp_err_t device_init() {
  gpio_set_direction(CONFIG_DEVICE_OUTPUT_GPIO, GPIO_MODE_OUTPUT);
  return ESP_OK;
}


static DeviceState device_get_state() {
  DeviceState state = {
    .on = gpio_get_level(CONFIG_DEVICE_OUTPUT_GPIO)
  };
  return state;
}

cJSON* device_execute(const char* const cmd, const cJSON *paramsJSON) 
{
  cJSON *state = cJSON_CreateObject();
  cJSON *root = cJSON_CreateObject();
  cJSON_AddBoolToObject(state, "online", true);


  ExecuteParams params;
  cJSON* params_on_item = cJSON_GetObjectItemCaseSensitive(paramsJSON, "on");
  if (!cJSON_IsBool(params_on_item)) {
    cJSON_AddStringToObject(root, "status", "ERROR");
    cJSON_AddStringToObject(root, "errorCode", "hardError");
    ESP_LOGE(DEVICE_TAG, "invalid 'on' parameter, is not boolean");
    return root;
  }
  if      (cJSON_IsTrue(params_on_item))  params.on = true;
  else if (cJSON_IsFalse(params_on_item)) params.on = false;
 

  if (strcmp(cmd, "action.devices.commands.OnOff")) {
    gpio_set_level(CONFIG_DEVICE_OUTPUT_GPIO, params.on);
    ESP_LOGI(
        DEVICE_TAG, "Changing GPIO %d state to %s", 
        CONFIG_DEVICE_OUTPUT_GPIO, params.on == false ? "false" : "true"
        );
    
    cJSON_AddBoolToObject(state, "on", params.on);
    cJSON_AddStringToObject(root, "status", "SUCCESS");
  } else {
    ESP_LOGE(DEVICE_TAG, "unrecognized command: %s", cmd);
    cJSON_AddBoolToObject(state, "on", device_get_state().on);
    cJSON_AddStringToObject(root, "status", "ERROR");
    cJSON_AddStringToObject(root, "errorCode", "functionNotSupported");
  }
  cJSON_AddItemToObject(root, "state", state);

  return root;
};

cJSON* device_query() {
  cJSON *root = cJSON_CreateObject();
  cJSON *state = cJSON_CreateObject();
  cJSON_AddStringToObject(root, "status", "SUCCESS");

  cJSON_AddBoolToObject(state, "online", true);
  cJSON_AddBoolToObject(state, "on", device_get_state().on);

  cJSON_AddItemToObject(root, "state", state);

  return root;
}

#endif
