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
	signedFromBase64, err := SignedFromBase64(signed.Base64())
	if err != nil {
		t.Fatalf("fail signed token from base64 %s", err.Error())
	}
	if !signedFromBase64.Equal(signed) {
		t.Fatalf("tokens not equal after b64 conv")
	}

	signatureFromBase64, err := SignatureFromBase64(signed.Signature().Base64())
	if err != nil {
		t.Fatalf("fail signature from base64 %s", err.Error())
	}
	if !signatureFromBase64.Equal(signed.Signature()) || !signatureFromBase64.Equal(signed.Signature()) {
		t.Fatalf("invlaid signature after base64 %s", err.Error())
	}

	payloadFromBase64, err := PayloadFromBase64(signed.Payload().Base64())
	if err != nil {
		t.Fatalf("fail payload from base64 %s", err.Error())
	}
	if !payloadFromBase64.Parse().Equal(validToken) || !payloadFromBase64.Equal(validToken.Payload()) {
		t.Fatalf("invlaid payload after base64 %s", err.Error())
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
