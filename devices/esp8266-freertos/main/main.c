#include <esp_log.h>
#include <freertos/FreeRTOS.h>
#include <esp_system.h>
#include <esp_netif.h>
#include <nvs_flash.h>
#include <esp_event.h>
#include "hf_io.h"
#include "hf_crypto.h"
#include "hf_mqtt.h"
#include "hf_wifi.h"

static const char *TAG = "app";

__unused void app_main() {
  ESP_LOGI(TAG, "[APP] Startup..");
  ESP_LOGI(TAG, "[APP] Free memory: %d bytes", esp_get_free_heap_size());
  ESP_LOGI(TAG, "[APP] IDF version: %s", esp_get_idf_version());

  ESP_ERROR_CHECK(nvs_flash_init());
  ESP_ERROR_CHECK(esp_netif_init());
  ESP_ERROR_CHECK(esp_event_loop_create_default());
  int err = crypto_init();
  if (err != ESP_OK) {
    return;
  }

  io_init();
  
  wifi_init_sta();

  mqtt_init();
}
