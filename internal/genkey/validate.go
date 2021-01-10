package genkey

import (
	"crypto/ed25519"
	crand "crypto/rand"
	"fmt"
	mrand "math/rand"
)

// ValidateKeypairsEquality checks if key are equal, key means pkey or skey, it must have Equal() property whcih returns boolean
func ValidateKeypairsEquality(pair1 Keypair, pair2 Keypair) {
	if !pair1.PublicKey.Equal(pair2.PublicKey) {
		panic(fmt.Errorf("Public keys are not equal"))
	}
	if !pair1.PrivateKey.Equal(pair2.PrivateKey) {
		panic(fmt.Errorf("Private keys are not equal"))
	}
}

// ValidateSigningPublicKey signs public key using private key and validates signature using public key
func ValidateSigningPublicKey(pkey ed25519.PublicKey, skey ed25519.PrivateKey) {
	signature := ed25519.Sign(skey, pkey)
	valid := ed25519.Verify(pkey, pkey, signature)
	if !valid {
		panic(fmt.Errorf("Couldn't verify the signature of signed public key"))
	}
}

// ValidateSigningRandomMessage signs public key using private key and validates signature using public key
func ValidateSigningRandomMessage(pkey ed25519.PublicKey, skey ed25519.PrivateKey) {
	msg := make([]byte, mrand.Intn(100000))
	crand.Read(msg) // write random messages to msg
	signature := ed25519.Sign(skey, pkey)
	valid := ed25519.Verify(pkey, pkey, signature)
	if !valid {
		panic(fmt.Errorf("Couldn't verify the signature of random message"))
	}
}
