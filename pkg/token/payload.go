package token

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/base64"
	"encoding/binary"
	"fmt"
)

const (
	// PayloadSize defines how many bytes there are in payload
	PayloadSize = UnixTimestampSize + TokenAudienceSize

	// PayloadBase64Size is size of Payload in Base64 format(with padding)
	PayloadBase64Size = (PayloadSize + 2) / 3 * 4
)

// Payload is payload of token
type Payload [PayloadSize]byte

// Sign returns signature of token
func (p Payload) Sign(key []byte) (Signed, error) {
	mac := hmac.New(sha256.New, key)
	_, err := mac.Write(p[:])
	if err != nil {
		return Signed{}, err
	}
	var sig Signature
	copy(sig[:], mac.Sum(nil))

	return p.Signed(sig), nil
}

// Equal checks equality of tokens
func (p Payload) Equal(p2 Payload) bool {
	return p == p2
}

// Parse parses token to struct
func (p Payload) Parse() Parsed {
	return Parsed{
		ExpiresAt: binary.BigEndian.Uint32(p[:4]),
		Audience:  p[4:],
	}
}

// Signed converts Payload to Signed token
func (p Payload) Signed(sig Signature) (signed Signed) {
	copy(signed[:SignatureSize], sig[:])
	copy(signed[SignatureSize:SignedSize], p[:])

	return signed
}

// Verify verifies if Payload is signed by signature passed via argument
func (p Payload) Verify(key []byte, sig Signature) error {
	return p.Signed(sig).Verify(key)
}

// Base64 encodes Payload to base64 format
func (p Payload) Base64() []byte {
	b64 := make([]byte, PayloadBase64Size)
	base64.StdEncoding.Encode(b64, p[:])
	return b64
}

// NewPayloadFromBase64 parses base64 payload and returns Payload
func NewPayloadFromBase64(b64 []byte) (p Payload, err error) {
	n, err := base64.StdEncoding.Decode(p[:], b64)
	if err != nil {
		return Payload{}, err
	}
	if n != PayloadSize {
		return Payload{}, fmt.Errorf("invalid payload size: %d", n)
	}
	return p, nil
}
