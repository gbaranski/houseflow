#ifndef HF_CRYPTO_H
#define HF_CRYPTO_H

typedef short int crypto_err_t;

// Errors which are negative are about bad configuration.
#define CRYPTO_ERR_OK 0
#define CRYPTO_ERR_BASE64_INVALID_CHARACTER -1
#define CRYPTO_ERR_BASE64_BUFFER_TOO_SMALL -2
#define CRYPTO_ERR_LENGTH_INVALID -3

#define ED25519_PKEY_LENGTH 32U
// 4 * ceil(ED25519_PKEY_LENGTH / 3) = 44
#define ED25519_BASE64_PKEY_LENGTH 44U

#define ED25519_SKEY_LENGTH (32U + 32U)
// 4 * ceil(ED25519_SKEY_LENGTH / 3) = 88
#define ED25519_BASE64_SKEY_LENGTH 88U

// Twice size of public key length
// https://ed25519.cr.yp.to/
#define ED25519_SIGNATURE_LENGTH 64U
// 4 * ceil(ED25519_SIGNATURE_LENGTH / 3) = 88
#define ED25519_BASE64_SIGNATURE_LENGTH 88U

struct Keypair {
  unsigned char pkey[ED25519_PKEY_LENGTH];
  // Thats seed
  unsigned char skey[ED25519_SKEY_LENGTH];
};

crypto_err_t get_private_key(struct Keypair *dst);
crypto_err_t get_public_key(struct Keypair *dst);

crypto_err_t sign_public_key(struct Keypair *kp, unsigned char *dst);

crypto_err_t encode_signature(const unsigned char *sig, unsigned char *dst);

#define CRYPTO_OK

#endif
