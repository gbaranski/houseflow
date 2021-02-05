package token

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/binary"
)

const (
	// BytesSize defines how many bytes there are in single bytes converted token
	BytesSize = UnixTimestampSize + TokenAudienceSize
)

// Bytes is type for not parsed token
type Bytes [BytesSize]byte

// Sign returns signature of token
func (b Bytes) Sign(key []byte) (Signed, error) {
	mac := hmac.New(sha256.New, key)
	_, err := mac.Write(b[:])
	if err != nil {
		return Signed{}, err
	}
	var sig Signature
	copy(sig[:], mac.Sum(nil))

	return b.Signed(sig), nil
}

// Equal checks equality of tokens
func (b Bytes) Equal(b2 Bytes) bool {
	return b == b2
}

// Parse parses token to struct
func (b Bytes) Parse() (p Parsed) {
	p.ExpiresAt = binary.BigEndian.Uint32(b[:4])
	copy(p.Audience[:], b[4:])
	return p
}

// Signed converts Bytes to Signed token
func (b Bytes) Signed(sig Signature) (signed Signed) {
	copy(signed[:SignatureSize], sig[:])
	copy(signed[SignatureSize:SignedSize], b[:])

	return signed
}

// Verify verifies if bytes are signed by signature passed via argument
func (b Bytes) Verify(key []byte, sig Signature) error {
	return b.Signed(sig).Verify(key)
}
