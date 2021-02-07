#include "hf_crypto.h"

#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include <esp_log.h>
#include <esp_netif.h>
#include <esp_system.h>
#include <esp_tls.h>
#include <mbedtls/base64.h>
#include <mqtt_client.h>
#include <nvs_flash.h>
#include <sodium.h>
#include <cJSON.h>

const unsigned char public_key[] = CONFIG_DEVICE_PUBLIC_KEY;
const unsigned char private_key[] = CONFIG_DEVICE_PRIVATE_KEY;

static Keypair g_kp;

int crypto_init()
{
  // Set public key here
  size_t pkey_len;
  int err = mbedtls_base64_decode(g_kp.pkey, ED25519_PKEY_BYTES, &pkey_len, public_key, ED25519_BASE64_PKEY_BYTES);
  if (err != 0)
  {
    ESP_LOGE(CRYPTO_TAG, "fail decode pkey err: %d", err);
    return err;
  }

  // Set private key here
  size_t skey_len;
  err = mbedtls_base64_decode(g_kp.skey, ED25519_SKEY_BYTES, &skey_len, private_key, ED25519_BASE64_SKEY_BYTES);
  if (err != 0)
  {
    ESP_LOGE(CRYPTO_TAG, "fail decode skey err: %d", err);
    return err;
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
  int err = crypto_sign_ed25519_detached(sig, NULL, g_kp.pkey, ED25519_PKEY_BYTES, g_kp.skey);
  if (err != 0)
  {
    return err;
  }
  return crypto_encode_signature(dst, sig);
}

int crypto_sign_payload(unsigned char *dst, const char *payload, const size_t payload_len)
{
  int err = crypto_sign_detached(dst, NULL, (const unsigned char *)payload, payload_len, g_kp.skey);
  if (err != 0)
  {
    ESP_LOGE(CRYPTO_TAG, "fail sign payload\n");
    return ESP_ERR_INVALID_RESPONSE;
  }

  return ESP_OK;
}