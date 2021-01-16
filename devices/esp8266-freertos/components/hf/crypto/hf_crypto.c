#include "hf_crypto.h"

#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "esp_log.h"
#include "esp_netif.h"
#include "esp_system.h"
#include "esp_tls.h"
#include "mbedtls/base64.h"
#include "mqtt_client.h"
#include "nvs_flash.h"
#include "sodium.h"

const unsigned char public_key[] = CONFIG_DEVICE_PUBLIC_KEY;
const unsigned char private_key[] = CONFIG_DEVICE_PRIVATE_KEY;

crypto_err_t get_public_key(struct Keypair *dst) {
  size_t olen;
  int err = mbedtls_base64_decode(dst->pkey, ED25519_PKEY_BYTES, &olen,
                                  public_key, ED25519_BASE64_PKEY_BYTES);

  if (err == MBEDTLS_ERR_BASE64_INVALID_CHARACTER)
    return CRYPTO_ERR_BASE64_INVALID_CHARACTER;
  else if (err == MBEDTLS_ERR_BASE64_BUFFER_TOO_SMALL)
    return CRYPTO_ERR_BASE64_BUFFER_TOO_SMALL;
  if (olen != ED25519_PKEY_BYTES) return CRYPTO_ERR_LENGTH_INVALID;

  return CRYPTO_ERR_OK;
}

crypto_err_t get_private_key(struct Keypair *dst) {
  size_t olen;
  int err = mbedtls_base64_decode(dst->skey, ED25519_SKEY_BYTES, &olen,
                                  private_key, ED25519_BASE64_SKEY_BYTES);

  if (err == MBEDTLS_ERR_BASE64_INVALID_CHARACTER)
    return CRYPTO_ERR_BASE64_INVALID_CHARACTER;
  else if (err == MBEDTLS_ERR_BASE64_BUFFER_TOO_SMALL)
    return CRYPTO_ERR_BASE64_BUFFER_TOO_SMALL;

  if (olen != ED25519_SKEY_BYTES) return CRYPTO_ERR_LENGTH_INVALID;
  return CRYPTO_ERR_OK;
}

crypto_err_t encode_signature(const unsigned char *sig, unsigned char *dst) {
  size_t olen;
  int err = mbedtls_base64_encode(dst, ED25519_BASE64_SIGNATURE_BYTES + 1,
                                  &olen, sig, ED25519_SIGNATURE_BYTES);

  if (err == MBEDTLS_ERR_BASE64_BUFFER_TOO_SMALL)
    return CRYPTO_ERR_BASE64_BUFFER_TOO_SMALL;

  if (olen != ED25519_BASE64_SIGNATURE_BYTES) return CRYPTO_ERR_LENGTH_INVALID;
  return CRYPTO_ERR_OK;
}