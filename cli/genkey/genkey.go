package genkey

import (
	"crypto/ed25519"
	crand "crypto/rand"
	"encoding/base64"
)

// GenerateKeypair generates ed25519 keypair, created it to be some higher level of abstraction
func GenerateKeypair() (ed25519.PublicKey, ed25519.PrivateKey) {
	pkey, skey, err := ed25519.GenerateKey(crand.Reader)
	if err != nil {
		panic(err)
	}
	return pkey, skey
}

// EncodeKeypair encodes both pkey and skey using base64 and returns two strings
//
// To be exact returned skey is seed, which is 2 times smaller than whole private key which stores pkey+seed
// https://pkg.go.dev/golang.org/x/crypto/ed25519#NewKeyFromSeed
func EncodeKeypair(pkey ed25519.PublicKey, skey ed25519.PrivateKey) (string, string) {
	pkeyEnc := base64.StdEncoding.EncodeToString(pkey)
	skeySeedEnc := base64.StdEncoding.EncodeToString(skey.Seed())
	return pkeyEnc, skeySeedEnc
}

// DecodeKeypair decodes base64 encoded pkey and seed of skey
func DecodeKeypair(pkeystr string, skeySeedstr string) (ed25519.PublicKey, ed25519.PrivateKey) {
	pkeydec, err := base64.StdEncoding.DecodeString(pkeystr)
	if err != nil {
		panic(err)
	}
	pkey := ed25519.PublicKey(pkeydec)

	skeySeed, err := base64.StdEncoding.DecodeString(skeySeedstr)
	if err != nil {
		panic(err)
	}
	skey := ed25519.NewKeyFromSeed(skeySeed)

	return pkey, skey
}
