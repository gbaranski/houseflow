#include <esp_log.h>
#include <freertos/FreeRTOS.h>
#include <esp_system.h>
#include <esp_netif.h>
#include <nvs_flash.h>
#include <esp_event.h>
#include <lwip/apps/sntp.h>
#include "hf_device.h"
#include "hf_io.h"
#include "hf_crypto.h"
#include "hf_mqtt.h"
#include "hf_wifi.h"

static const char *TAG = "app";

// some year, used to check if retrieved date is real or not
#define FIXED_YEAR 2021
#define SNTP_MAX_RETRY 10

esp_err_t time_init() 
{
  ESP_LOGI(TAG, "Initializng SNTP");
  sntp_setoperatingmode(SNTP_OPMODE_POLL);
  sntp_setservername(0, "0.europe.pool.ntp.org");
  sntp_init();

  setenv("TZ", "UTC", 1);
  tzset();

  time_t now = 0;
  struct tm timeinfo;

  int retry = 0;

  while(timeinfo.tm_year < (FIXED_YEAR-1900) && retry < SNTP_MAX_RETRY) {
    ESP_LOGI(TAG, "waiting for time to initialize, retry %d/%d", retry, SNTP_MAX_RETRY);
    time(&now);
    localtime_r(&now, &timeinfo);

    vTaskDelay(1000 / portTICK_RATE_MS);
  }
  if (timeinfo.tm_year < (FIXED_YEAR-1900)) {
    ESP_LOGE(TAG, "timeout waiting for SNTP time ts: %ld", time(&now));
    return ESP_ERR_TIMEOUT;
  }
  ESP_LOGI(TAG, "date initialized, timestamp: %ld", now);

  return ESP_OK;
}

void app_main()
{
  ESP_LOGI(TAG, "[APP] Startup..");
  ESP_LOGI(TAG, "[APP] Free memory: %d bytes", esp_get_free_heap_size());
  ESP_LOGI(TAG, "[APP] IDF version: %s", esp_get_idf_version());

  ESP_ERROR_CHECK(nvs_flash_init());
  ESP_ERROR_CHECK(esp_netif_init());
  ESP_ERROR_CHECK(esp_event_loop_create_default());
  ESP_ERROR_CHECK(device_init());
  ESP_ERROR_CHECK(crypto_init());
  wifi_init_sta();
  ESP_ERROR_CHECK(time_init());

  ESP_ERROR_CHECK(mqtt_init());
}
