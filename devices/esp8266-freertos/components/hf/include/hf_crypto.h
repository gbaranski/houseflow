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


// ED25519_SIGNATURE_BYTES + sizeof(uint32_t);
#define PASSWORD_BYTES 68
// ((4 * PASSWORD_BYTES / 3) + 3) & ~3;
#define PASSWORD_BASE64_BYTES 92

#define REQUEST_ID_SIZE 16U

esp_err_t crypto_init();

esp_err_t crypto_encode_signature(unsigned char *dst, const unsigned char *sig);
esp_err_t crypto_generate_password( unsigned char* dst );
esp_err_t crypto_sign_payload(unsigned char *dst, const char *payload, const size_t payload_len);
esp_err_t crypto_encode_password( unsigned char* dst, const unsigned char* const src );

// verifies if requestID with Data is valid by checking with signature, data can be NULL
bool crypto_verify_server_payload( const char* sig, const uint8_t* requestID, const char* body, const size_t body_len ); 

#endif
