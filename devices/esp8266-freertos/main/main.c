/* WiFi station Example

   This example code is in the Public Domain (or CC0 licensed, at your option.)

   Unless required by applicable law or agreed to in writing, this
   software is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
   CONDITIONS OF ANY KIND, either express or implied.
*/
#include "esp_log.h"
#include "nvs.h"
#include "nvs_flash.h"

#include "houseflow/wifi.h"

void app_main()
{
    ESP_ERROR_CHECK(nvs_flash_init());

    ESP_LOGI(WIFI_TAG, "ESP_WIFI_MODE_STA");
    wifi_init_sta();
}
