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

int crypto_encode_signature(const unsigned char *sig, unsigned char *dst)
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
  unsigned char password[ED25519_SIGNATURE_BYTES];
  int err = crypto_sign_ed25519_detached(dst, NULL, g_kp.pkey, ED25519_PKEY_BYTES, g_kp.skey);
  if (err != 0)
  {
    return err;
  }
  return crypto_encode_signature(password, dst);
}

int crypto_sign_response_combined(char *dst, DeviceResponse *res, const char *res_str)
{

  // fix this buffer later, not sure if we need that big buffer for just signature
  unsigned char res_sig[ED25519_SIGNATURE_BYTES];
  int err = crypto_sign_detached(res_sig, NULL, (const unsigned char *)res_str, strlen(res_str), g_kp.skey);
  if (err != 0)
  {
    printf("fail sign code: %d\n", err);
    return ESP_ERR_INVALID_RESPONSE;
  }
  unsigned char res_sig_encoded[ED25519_BASE64_SIGNATURE_BYTES];
  err = crypto_encode_signature(res_sig, res_sig_encoded);
  if (err != ESP_OK)
  {
    printf("fail encode response sig %d\n", err);
    // ESP_LOGE("fail encode signature %d",(int)crypto_err);
    return ESP_ERR_INVALID_RESPONSE;
  }
  printf("response signature: %s\n", res_sig_encoded);

  strcpy(dst, (const char *)res_sig_encoded);
  strcat(dst, ".");
  strcat(dst, res_str);
  return ESP_OK;
}