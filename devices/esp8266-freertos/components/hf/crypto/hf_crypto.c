#include "hf_crypto.h"
#include "sdkconfig.h"

#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <FreeRTOS.h>

#include <esp_log.h>
#include <esp_err.h>
#include <esp_netif.h>
#include <esp_system.h>
#include <esp_tls.h>
#include <mbedtls/base64.h>
#include <mqtt_client.h>
#include <nvs_flash.h>
#include <sodium.h>
#include <cJSON.h>
#include <lwip/apps/sntp.h>

static unsigned char server_public_key[ED25519_PKEY_BYTES];
static unsigned char public_key[ED25519_PKEY_BYTES];
static unsigned char private_key[ED25519_SKEY_BYTES];

int crypto_init()
{
  // Set public key here
  size_t pkey_len;
  int err = mbedtls_base64_decode(public_key, ED25519_PKEY_BYTES, &pkey_len, (const unsigned char*)CONFIG_DEVICE_PUBLIC_KEY, ED25519_BASE64_PKEY_BYTES);
  if (err != 0)
  {
    ESP_LOGE(CRYPTO_TAG, "fail decode pkey err: %d", err);
    return err;
  }
  if (pkey_len != ED25519_PKEY_BYTES) {
    ESP_LOGE(CRYPTO_TAG, "invalid decoded public key length: %zu", pkey_len);
    return ESP_ERR_INVALID_SIZE;
  }

  // Set private key here
  size_t skey_len;
  err = mbedtls_base64_decode(private_key, ED25519_SKEY_BYTES, &skey_len, (const unsigned char*)CONFIG_DEVICE_PRIVATE_KEY, ED25519_BASE64_SKEY_BYTES);
  if (err != 0)
  {
    ESP_LOGE(CRYPTO_TAG, "fail decode skey err: %d", err);
    return err;
  }
  if (skey_len != ED25519_SKEY_BYTES) {
    ESP_LOGE(CRYPTO_TAG, "invalid decoded private key length: %zu", skey_len);
    return ESP_ERR_INVALID_SIZE;
  }

  // Set public key here
  size_t server_pkey_len;
  err = mbedtls_base64_decode(server_public_key, ED25519_PKEY_BYTES, &server_pkey_len, (const unsigned char*)CONFIG_SERVER_PUBLIC_KEY, ED25519_BASE64_PKEY_BYTES);
  if (err != 0)
  {
    ESP_LOGE(CRYPTO_TAG, "fail decode server pkey err: %d", err);
    return err;
  }
  if (server_pkey_len != ED25519_PKEY_BYTES) {
    ESP_LOGE(CRYPTO_TAG, "invalid decoded server public key length: %zu", pkey_len);
    return ESP_ERR_INVALID_SIZE;
  }

  ESP_LOGI(CRYPTO_TAG, "Initializng SNTP");
  sntp_setoperatingmode(SNTP_OPMODE_POLL);
  sntp_setservername(0, "pool.ntp.org");
  sntp_init();
  setenv("TZ", "UTC+1", 1);
  tzset();

  while(true) {
    time_t now;
    time_t nowUTC;
    struct tm nowtm;
    struct tm nowtmUTC;
    time(&now);
    localtime_r(&now, &nowtm);
    gmtime_r(&nowUTC, &nowtmUTC);
    ESP_LOGI(CRYPTO_TAG, "waiting for time");
    ESP_LOGI(CRYPTO_TAG, "timestamp: %ld", now);
    ESP_LOGI(CRYPTO_TAG, "UTC timestamp: %ld", nowUTC);
    ESP_LOGI(CRYPTO_TAG, "time(): %ld", time(NULL));

    ESP_LOGI(CRYPTO_TAG, "year: %d", nowtm.tm_year);
    ESP_LOGI(CRYPTO_TAG, "month: %d", nowtm.tm_mon);
    ESP_LOGI(CRYPTO_TAG, "day: %d", nowtm.tm_yday);
    ESP_LOGI(CRYPTO_TAG, "hour: %d", nowtm.tm_hour);
    ESP_LOGI(CRYPTO_TAG, "minute: %d", nowtm.tm_min);
    vTaskDelay(1000 / portTICK_RATE_MS);
  }

  time_t now;
  struct tm time;
  localtime_r(&now, &time);
  ESP_LOGI(CRYPTO_TAG, "timestamp: %ld now: %ld", mktime(&time), now);

  return ESP_OK;
}

int crypto_encode_signature(unsigned char *dst, const unsigned char *sig)
{
  size_t olen;
  int err = mbedtls_base64_encode(dst, ED25519_BASE64_SIGNATURE_BYTES + 1,
                                  &olen, sig, ED25519_SIGNATURE_BYTES);

  if (err == MBEDTLS_ERR_BASE64_BUFFER_TOO_SMALL)
    return MBEDTLS_ERR_BASE64_BUFFER_TOO_SMALL;

  if (olen != ED25519_BASE64_SIGNATURE_BYTES)
    return ESP_ERR_INVALID_SIZE;

  return ESP_OK;
}

int crypto_generate_password( unsigned char* dst )
{
  time_t now = 0;
  time(&now);
  ESP_LOGI(CRYPTO_TAG, "timestamp: %ld", now);

  uint8_t ts[4];
  ts[0] = (now >> 24) & 0xFF;
  ts[1] = (now >> 16) & 0xFF;
  ts[2] = (now >> 8) & 0xFF;
  ts[3] = now & 0xFF;

  unsigned char sig[ED25519_SIGNATURE_BYTES];
  int err = crypto_sign_ed25519_detached( sig, NULL, ts, sizeof(ts), private_key );
  if (err != 0) return err;

  memcpy( dst, sig, ED25519_SIGNATURE_BYTES );
  memcpy( &dst[ED25519_SIGNATURE_BYTES], ts, sizeof(ts) );

  return 0;
}

int crypto_encode_password( unsigned char* dst, const unsigned char* const src ) 
{
  size_t olen;
  return mbedtls_base64_encode(dst, PASSWORD_BASE64_BYTES + 1,
                                  &olen, src, PASSWORD_BYTES);
}

int crypto_sign_payload(unsigned char *dst, const char *payload, const size_t payload_len)
{
  int err = crypto_sign_detached(dst, NULL, (const unsigned char *)payload, payload_len, private_key);
  if (err != 0)
  {
    ESP_LOGE(CRYPTO_TAG, "fail sign payload\n");
    return ESP_ERR_INVALID_RESPONSE;
  }

  return ESP_OK;
}

// verifies if requestID with Data is valid by checking with signature, body can be NULL
bool crypto_verify_server_payload( const char* sig, const uint8_t* requestID, const char* body, const size_t body_len ) 
{
  char p[REQUEST_ID_SIZE + body_len];
  memcpy( p, requestID, REQUEST_ID_SIZE );
  if ( body != NULL && body_len > 0 ) {
    memcpy( &p[REQUEST_ID_SIZE], body, body_len );
  }

  int err = crypto_sign_verify_detached( (const unsigned char *)sig, (const unsigned char*)p, REQUEST_ID_SIZE + body_len, server_public_key );
  return err == 0;
}
