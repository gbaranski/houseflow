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
func EncodeKeypair(pkey ed25519.PublicKey, skey ed25519.PrivateKey) (string, string) {
	pkeyEnc := base64.StdEncoding.EncodeToString(pkey)
	skeyEnc := base64.StdEncoding.EncodeToString(skey)
	return pkeyEnc, skeyEnc
}

// DecodeKeypair decodes base64 encoded pkey and skey
func DecodeKeypair(pkeystr string, skeySeedstr string) (ed25519.PublicKey, ed25519.PrivateKey) {
	pkeydec, err := base64.StdEncoding.DecodeString(pkeystr)
	if err != nil {
		panic(err)
	}

	skeydec, err := base64.StdEncoding.DecodeString(skeySeedstr)
	if err != nil {
		panic(err)
	}

	return ed25519.PublicKey(pkeydec),
		ed25519.PrivateKey(skeydec)
}
