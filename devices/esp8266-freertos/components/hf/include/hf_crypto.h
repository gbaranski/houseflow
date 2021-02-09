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

#define REQUEST_ID_SIZE 16U

int crypto_init();

int crypto_encode_signature(unsigned char *dst, const unsigned char *sig);
int crypto_generate_password(unsigned char *dst);
int crypto_sign_payload(unsigned char *dst, const char *payload, const size_t payload_len);

// verifies if requestID with Data is valid by checking with signature, data can be NULL
bool crypto_verify_server_payload( const char* sig, const uint8_t* requestID, const char* body, const size_t body_len ); 

#endif
