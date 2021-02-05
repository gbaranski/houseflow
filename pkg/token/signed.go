package token

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/base64"
	"fmt"
	"time"
)

const (
	// SignedSize is size of signed token
	SignedSize = SignatureSize + PayloadSize

	// SignedBase64Size is size of Signed token in Base64 format(with padding)
	SignedBase64Size = (SignedSize + 2) / 3 * 4
)

// Signed is signed token
type Signed [SignedSize]byte

// Signature converts Signed token to signature
func (s Signed) Signature() (sig Signature) {
	copy(sig[:], s[:SignatureSize])
	return sig
}

// Payload returns payload from signed token
func (s Signed) Payload() (p Payload) {
	copy(p[:], s[SignatureSize:SignatureSize+PayloadSize])
	return p
}

// Verify verifies token and returns nil if its valid
func (s Signed) Verify(key []byte) error {
	b, sig := s.Payload(), s.Signature()
	mac := hmac.New(sha256.New, key)
	_, err := mac.Write(b[:])
	if err != nil {
		return fmt.Errorf("fail write hmac %s", err.Error())
	}
	parsed := b.Parse()
	if int64(parsed.ExpiresAt) < time.Now().Unix() {
		return fmt.Errorf("token is outdated by %s", time.Since(time.Unix(int64(parsed.ExpiresAt), 0)).String())
	}
	if !hmac.Equal(sig[:], mac.Sum(nil)) {
		return fmt.Errorf("signature is invalid")
	}
	return nil
}

// Parse converts signed token to parsed token
func (s Signed) Parse() Parsed {
	return s.Payload().Parse()
}

// Equal checks equality with another signed token
func (s Signed) Equal(s2 Signed) bool {
	return s == s2
}

// Base64 encodes signed token to base64 format
func (s Signed) Base64() (b64 [SignedBase64Size]byte) {
	base64.RawStdEncoding.Encode(b64[:SignedBase64Size], s[:])
	return
}

// NewSignedFromBase64 parses base64 and returns it in signed format
func NewSignedFromBase64(b64 [SignedBase64Size]byte) (s Signed, err error) {
	n, err := base64.StdEncoding.Decode(s[:], b64[:])
	if err != nil {
		return Signed{}, err
	}
	if n != SignedSize {
		return Signed{}, fmt.Errorf("invalid token size: %d", n)
	}
	return s, nil
}
