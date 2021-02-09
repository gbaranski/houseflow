#include "hf_crypto.h"

#include <stdint.h>
#include <stdio.h>
#include <string.h>

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

static const unsigned char public_key_base64[] = CONFIG_DEVICE_PUBLIC_KEY;
static const unsigned char private_key_base64[] = CONFIG_DEVICE_PRIVATE_KEY;
static const unsigned char server_public_key_base64[] = CONFIG_SERVER_PUBLIC_KEY;

static unsigned char server_public_key[ED25519_PKEY_BYTES];
static unsigned char public_key[ED25519_PKEY_BYTES];
static unsigned char private_key[ED25519_SKEY_BYTES];

int crypto_init()
{
  // Set public key here
  size_t pkey_len;
  int err = mbedtls_base64_decode(public_key, ED25519_PKEY_BYTES, &pkey_len, public_key_base64, ED25519_BASE64_PKEY_BYTES);
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
  err = mbedtls_base64_decode(private_key, ED25519_SKEY_BYTES, &skey_len, private_key_base64, ED25519_BASE64_SKEY_BYTES);
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
  err = mbedtls_base64_decode(server_public_key, ED25519_PKEY_BYTES, &server_pkey_len, server_public_key_base64, ED25519_BASE64_PKEY_BYTES);
  if (err != 0)
  {
    ESP_LOGE(CRYPTO_TAG, "fail decode server pkey err: %d", err);
    return err;
  }
  if (server_pkey_len != ED25519_PKEY_BYTES) {
    ESP_LOGE(CRYPTO_TAG, "invalid decoded server public key length: %zu", pkey_len);
    return ESP_ERR_INVALID_SIZE;
  }

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

int crypto_generate_password(unsigned char *dst)
{
  unsigned char sig[ED25519_SIGNATURE_BYTES];
  int err = crypto_sign_ed25519_detached(sig, NULL, public_key, ED25519_PKEY_BYTES, private_key);
  if (err != 0)
  {
    return err;
  }
  return crypto_encode_signature(dst, sig);
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
