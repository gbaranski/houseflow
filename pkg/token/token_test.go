package token

import (
	"os"
	"testing"
	"time"

	"github.com/gbaranski/houseflow/pkg/utils"
)

var (
	key        = []byte(utils.GenerateRandomString(20))
	validToken = Parsed{
		ExpiresAt: uint32(time.Now().Add(time.Hour).Unix()),
	}
	expiredToken = Parsed{
		ExpiresAt: uint32(time.Now().Unix() - 3600),
	}
)

func TestMain(m *testing.M) {
	copy(validToken.Audience[:], utils.GenerateRandomString(36))
	copy(expiredToken.Audience[:], utils.GenerateRandomString(36))
	os.Exit(m.Run())
}

func TestValidCreateToken(t *testing.T) {
	signed, err := validToken.Sign(key)
	if err != nil {
		t.Fatalf(err.Error())
	}

	err = signed.Verify(key)
	if err != nil {
		t.Fatalf("fail verify token: %s", err.Error())
	}
	if !signed.Parse().Equal(validToken) {
		t.Fatalf("tokens are not equal")
	}
	if !signed.Parse().Payload().Signed(signed.Signature()).Equal(signed) {
		t.Fatalf("token malformed after multiple conversions")
	}
	signedFromBase64, err := NewSignedFromBase64(signed.Base64())
	if err != nil {
		t.Fatalf("fail signed token from base64 %s", err.Error())
	}
	if !signedFromBase64.Equal(signed) {
		t.Fatalf("tokens not equal after b64 conv")
	}

	signatureFromBase64, err := NewSignatureFromBase64(signed.Signature().Base64())
	if err != nil {
		t.Fatalf("fail signature from base64 %s", err.Error())
	}
	if !signatureFromBase64.Equal(signed.Signature()) || !signatureFromBase64.Equal(signed.Signature()) {
		t.Fatalf("invalid signature after base64 %s", err.Error())
	}

	payloadFromBase64, err := NewPayloadFromBase64(signed.Payload().Base64())
	if err != nil {
		t.Fatalf("fail payload from base64 %s", err.Error())
	}
	if !payloadFromBase64.Parse().Equal(validToken) || !payloadFromBase64.Equal(validToken.Payload()) {
		t.Fatalf("invalid payload after base64 %s", err.Error())
	}

}

func TestExpiredCreateToken(t *testing.T) {
	signed, err := expiredToken.Sign(key)
	if err != nil {
		t.Fatalf(err.Error())
	}
	err = signed.Verify(key)
	if err == nil {
		t.Fatalf("expected to fail verification")
	}
	if !signed.Parse().Equal(expiredToken) {
		t.Fatalf("tokens are not equal")
	}
}

func BenchmarkSignToken(b *testing.B) {
	for i := 0; i < b.N; i++ {
		_, err := validToken.Sign(key)
		if err != nil {
			b.Fatal(err)
		}
	}
}

func BenchmarkVerifyToken(b *testing.B) {
	b.StopTimer()
	signed, err := validToken.Sign(key)
	if err != nil {
		b.Fatal(err)
	}
	b.StartTimer()

	for i := 0; i < b.N; i++ {
		if err := signed.Verify(key); err != nil {
			b.Fatalf("fail verify %s", err.Error())
		}
	}
}

func BenchmarkSignVerifyToken(b *testing.B) {
	for i := 0; i < b.N; i++ {
		signed, err := validToken.Sign(key)
		if err != nil {
			b.Fatalf("fail sig %s", err.Error())
		}
		if err = signed.Verify(key); err != nil {
			b.Fatalf("fail verify %s", err.Error())
		}
	}
}

func BenchmarkVerifySignedBase64(b *testing.B) {
	b.StopTimer()
	signed, err := validToken.Sign(key)
	if err != nil {
		b.Fatal(err)
	}
	b.StartTimer()

	for i := 0; i < b.N; i++ {
		s, err := NewSignedFromBase64(signed.Base64())
		if err != nil {
			b.Fatalf("fail convert signed from base64")
		}
		if s.Verify(key) != nil {
			b.Fatalf("fail verify token %s", err.Error())
		}
	}
}
