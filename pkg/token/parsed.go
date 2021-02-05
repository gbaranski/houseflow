package token

// Parsed is parsed token
type Parsed struct {
	// Must have fixed size of TokenAudienceSize
	Audience [TokenAudienceSize]byte
	// ExpiresAt is unix timestamp
	ExpiresAt uint32
}

// Bytes converts token to bytes and returns it
func (t Parsed) Bytes() (b Bytes) {
	b[0] = byte(t.ExpiresAt >> 24)
	b[1] = byte(t.ExpiresAt >> 16)
	b[2] = byte(t.ExpiresAt >> 8)
	b[3] = byte(t.ExpiresAt)
	copy(b[4:TokenAudienceSize+4], t.Audience[:])

	return b
}

// Sign converts token to bytes and then signs it and returns the signed token
func (t Parsed) Sign(key []byte) (token Signed, err error) {
	b := t.Bytes()
	sig, err := b.Sign(key)

	copy(token[:SignatureSize], sig[:])
	copy(token[SignatureSize:SignedSize], b[:])

	return token, nil
}

// Verify verifies if parsed token is signed by signature passed via argument
func (t Parsed) Verify(key []byte, sig Signature) error {
	return t.Bytes().Signed(sig).Verify(key)
}

// Equal checks equality of tokens
func (t Parsed) Equal(t2 Parsed) bool {
	return t.Audience == t2.Audience && t.ExpiresAt == t2.ExpiresAt
}
