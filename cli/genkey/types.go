package genkey

import "crypto/ed25519"

// Keypair contains both ed25519 pkey and skey
type Keypair struct {
	PublicKey  ed25519.PublicKey
	PrivateKey ed25519.PrivateKey
}
