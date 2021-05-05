#include <esp_err.h>
#include <cJSON.h>

#define DEVICE_TAG "device"

#define cJSON_AddNumberToObjectSafe(object, name, number) {\
  if (cJSON_AddNumberToObject(object, name, number) == NULL) {\
    ESP_LOGE(DEVICE_TAG, "fail adding number '%s' to object", name);\
    return NULL;\
  }\
}\

#define cJSON_AddBoolToObjectSafe(object, name, boolean) {\
  if (cJSON_AddBoolToObject(object, name, boolean) == NULL) {\
    ESP_LOGE(DEVICE_TAG, "fail adding boolean '%s' to object", name);\
    return NULL;\
  }\
}\

#define cJSON_AddStringToObjectSafe(object, name, string) {\
  if (cJSON_AddStringToObject(object, name, string) == NULL) {\
    ESP_LOGE(DEVICE_TAG, "fail adding string '%s' to object", name);\
    return NULL;\
  }\
}\

esp_err_t device_init();
cJSON* device_execute(const char* const cmd, const cJSON *params);
cJSON* device_query();

