#ifndef HF_CRYPTO_H
#define HF_CRYPTO_H
#include "hf_types.h"

#define CRYPTO_TAG "crypto"

#define ED25519_PKEY_BYTES 32U
// 4 * ceil(ED25519_PKEY_LENGTH / 3) = 44
#define ED25519_BASE64_PKEY_BYTES 44U

#define ED25519_SKEY_BYTES (32U + 32U)
// 4 * ceil(ED25519_SKEY_LENGTH / 3) = 88
#define ED25519_BASE64_SKEY_BYTES 88U

// Twice size of public key length
// https://ed25519.cr.yp.to/
#define ED25519_SIGNATURE_BYTES 64U
// 4 * ceil(ED25519_SIGNATURE_LENGTH / 3) = 88
#define ED25519_BASE64_SIGNATURE_BYTES 88U

typedef struct {
  unsigned char pkey[ED25519_PKEY_BYTES];
  // Thats seed
  unsigned char skey[ED25519_SKEY_BYTES];
} Keypair;


int crypto_init();

int crypto_encode_signature(const unsigned char *sig, unsigned char *dst);
int crypto_generate_password(unsigned char* dst);
int crypto_sign_response_combined(char* dst, DeviceResponse *res, const char* res_str);

#endif
