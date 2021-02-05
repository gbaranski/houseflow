package token

import (
	"crypto/hmac"
	"crypto/sha256"
	"fmt"
	"time"
)

const (
	// SignedSize is size of signed token
	SignedSize = SignatureSize + BytesSize
)

// Signed is signed token
type Signed [SignedSize]byte

// Signature converts Signed token to signature
func (s Signed) Signature() (sig Signature) {
	copy(sig[:], s[:SignatureSize])
	return sig
}

// Bytes converts Signed token to bytes
func (s Signed) Bytes() (b Bytes) {
	copy(b[:], s[SignatureSize:SignatureSize+BytesSize])
	return b
}

// Verify verifies token and returns nil if its valid
func (s Signed) Verify(key []byte) error {
	b, sig := s.Bytes(), s.Signature()
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
	return s.Bytes().Parse()
}
