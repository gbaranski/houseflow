#include <esp_err.h>
#include <cJSON.h>

#define DEVICE_TAG "device"

esp_err_t device_init();
cJSON* device_execute(const char* const cmd, const cJSON *params);
cJSON* device_query();
