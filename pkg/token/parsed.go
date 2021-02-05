package token

import (
	"bytes"
)

// Parsed is parsed token
type Parsed struct {
	// Audience Must have fixed size of TokenAudienceSize
	Audience []byte
	// ExpiresAt is unix timestamp
	ExpiresAt uint32
}

// Payload retursn payload of the token
func (t Parsed) Payload() (b Payload) {
	b[0] = byte(t.ExpiresAt >> 24)
	b[1] = byte(t.ExpiresAt >> 16)
	b[2] = byte(t.ExpiresAt >> 8)
	b[3] = byte(t.ExpiresAt)
	copy(b[4:TokenAudienceSize+4], t.Audience[:])

	return b
}

// Sign takes token Payload and then signs it and returns the signed token
func (t Parsed) Sign(key []byte) (token Signed, err error) {
	p := t.Payload()
	sig, err := p.Sign(key)

	if err != nil {
		return Signed{}, err
	}

	copy(token[:SignatureSize], sig[:])
	copy(token[SignatureSize:SignedSize], p[:])

	return token, nil
}

// Signed takes payload at appends it after signature
func (t Parsed) Signed(sig Signature) (s Signed) {
	return t.Payload().Signed(sig)
}

// Verify verifies if parsed token is signed by signature passed via argument
func (t Parsed) Verify(key []byte, sig Signature) error {
	return t.Payload().Signed(sig).Verify(key)
}

// Equal checks equality of tokens
func (t Parsed) Equal(t2 Parsed) bool {
	return bytes.Equal(t.Audience, t2.Audience) && t.ExpiresAt == t2.ExpiresAt
}
