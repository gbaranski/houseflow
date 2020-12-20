#include <esp_types.h>
#include <esp_log.h>
#include <nvs.h>
#include <nvs_flash.h>
#include <esp_system.h>
#include "esp_event.h"
#include "esp_netif.h"

#include "hf_wifi.h"
#include "hf_mqtt.h"

static const char *TAG = "app";

__unused void app_main() {
    ESP_LOGI(TAG, "[APP] Startup..");
    ESP_LOGI(TAG, "[APP] Free memory: %d bytes", esp_get_free_heap_size());
    ESP_LOGI(TAG, "[APP] IDF version: %s", esp_get_idf_version());

    ESP_ERROR_CHECK(nvs_flash_init());
    ESP_ERROR_CHECK(esp_netif_init());
    ESP_ERROR_CHECK(esp_event_loop_create_default());

    wifi_init_sta();
    mqtt_connect();
}
