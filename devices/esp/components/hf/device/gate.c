#include "esp_system.h"
#include "sdkconfig.h"

#if CONFIG_DEVICE_TYPE_GATE == 1

#include "hf_device.h"

#include <string.h>
#include <freertos/FreeRTOS.h>
#include <freertos/task.h>
#include <stdbool.h>
#include <esp_err.h>
#include <esp_log.h>
#include <driver/gpio.h>
#include <cJSON.h>


typedef struct {
} DeviceState;

typedef struct {
  uint8_t open_percent;
} ExecuteParams;

static void device_open_close_task( void *pv_params ) 
{
  ExecuteParams *execute_params = (ExecuteParams*)pv_params;
  ESP_LOGI(
      DEVICE_TAG, "Changing GPIO %d state to %s", 
      CONFIG_DEVICE_OUTPUT_GPIO, execute_params->open_percent == 100U ? "true" : "false"
      );
  gpio_set_level(CONFIG_DEVICE_OUTPUT_GPIO, execute_params->open_percent == 100U);
  vTaskDelay( 1000 / portTICK_PERIOD_MS );
  gpio_set_level(CONFIG_DEVICE_OUTPUT_GPIO, execute_params->open_percent != 100U);
  ESP_LOGI(
      DEVICE_TAG, "Changing GPIO %d state to %s", 
      CONFIG_DEVICE_OUTPUT_GPIO, execute_params->open_percent != 100U ? "true" : "false"
      );
  vTaskDelete( NULL );
}

esp_err_t device_init() {
  gpio_set_direction(CONFIG_DEVICE_OUTPUT_GPIO, GPIO_MODE_OUTPUT);
  return ESP_OK;
}

cJSON* device_execute(const char* const cmd, const cJSON *paramsJSON) 
{
  cJSON *root = cJSON_CreateObject();
  cJSON *state = cJSON_CreateObject();
  if (root == NULL) {
    ESP_LOGE(DEVICE_TAG, "fail root object == NULL");
    return NULL;
  }
  if (state == NULL) {
    ESP_LOGE(DEVICE_TAG, "fail state object == NULL");
    return NULL;
  }

  cJSON_AddBoolToObjectSafe(state, "online", true);

  cJSON* params_open_percent_item = cJSON_GetObjectItemCaseSensitive( paramsJSON, "openPercent" );
  if ( !cJSON_IsNumber( params_open_percent_item ) ) {
    ESP_LOGE( DEVICE_TAG, "invalid 'open_percent' parameter, is not number" );
    cJSON_AddStringToObjectSafe( root, "status", "ERROR" );
    cJSON_AddStringToObjectSafe( root, "errorCode", "hardError" );
    return root;
  }
  ExecuteParams params = { 
    .open_percent = params_open_percent_item->valueint,
  };
 
  if (strcmp(cmd, "action.devices.commands.OpenClose")) {
    /* xTaskCreate( */
    /*     device_open_close_task, */
    /*     "open_close", */
    /*     configMINIMAL_STACK_SIZE + sizeof(ExecuteParams*), */
    /*     ( void* ) &params, */
    /*     tskIDLE_PRIORITY+1, */
    /*     NULL */
    /*     ); */
    cJSON_AddStringToObjectSafe( root, "status", "SUCCESS" );

    
    if ( cJSON_AddNumberToObject( state, "openPercent", 100 ) == NULL ) {
      ESP_LOGE( DEVICE_TAG, "adding open_percent returned NULL" );
      return NULL;
    }
  } else {
    ESP_LOGE(DEVICE_TAG, "unrecognized command: %s", cmd);
    cJSON_AddNumberToObjectSafe(state, "openPercent", 0);
    cJSON_AddStringToObjectSafe(root, "status", "ERROR");
    cJSON_AddStringToObjectSafe(root, "errorCode", "functionNotSupported");
  }
  cJSON_AddItemToObject( root, "state", state );

  return root;
};

cJSON* device_query() {
  cJSON *root = cJSON_CreateObject();
  cJSON *state = cJSON_CreateObject();
  if (root == NULL || state == NULL) {
    ESP_LOGE(DEVICE_TAG, "fail creating root or state object");
    return NULL;
  }

  cJSON_AddStringToObjectSafe(root, "status", "SUCCESS");

  cJSON_AddBoolToObjectSafe(state, "online", true);
  cJSON_AddNumberToObjectSafe(state, "openPercent", 0.0f);

  cJSON_AddItemToObject(root, "state", state);

  return root;
}

#endif

