#include <esp_types.h>
#include "esp_log.h"
#include "nvs.h"
#include "nvs_flash.h"

#include "hf_wifi.h"

__unused void app_main()
{
    ESP_ERROR_CHECK(nvs_flash_init());

    ESP_LOGI(WIFI_TAG, "ESP_WIFI_MODE_STA");
    wifi_init_sta();
}
