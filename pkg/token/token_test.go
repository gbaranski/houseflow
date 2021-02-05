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
