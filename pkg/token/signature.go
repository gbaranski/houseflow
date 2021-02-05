package token

import "crypto/sha256"

const (
	// SignatureSize is size of token signature
	SignatureSize = sha256.Size
)

// Signature is only signature of token
type Signature [SignatureSize]byte
