package utils

import (
	"fmt"
	"testing"
	"time"

	"bou.ke/monkey"
	"github.com/google/uuid"
)

func TestValidCreateToken(t *testing.T) {
	key := GenerateRandomString(20)
	aud := GenerateRandomString(20)
	tokenID, err := uuid.NewRandom()
	if err != nil {
		t.Fatalf(err.Error())
	}
	token := Token{
		ExpiresAt: time.Now().Add(time.Hour).Unix(),
		Audience:  aud,
		ID:        tokenID.String(),
	}
	strtoken, err := token.Sign([]byte(key))
	fmt.Printf("strtoken: %s\n", strtoken)
	if err != nil {
		t.Fatalf(err.Error())
	}
	dt, err := VerifyToken(strtoken, []byte(key))
	if err != nil {
		t.Fatalf("fail verify token: %s", err.Error())
	}
	if !dt.Equal(token) {
		t.Fatalf("fail tokens are not equal")
	}
}

func TestExpiredCreateToken(t *testing.T) {
	key := GenerateRandomString(20)
	aud := GenerateRandomString(20)
	tokenID, err := uuid.NewRandom()
	if err != nil {
		t.Fatalf(err.Error())
	}
	token := Token{
		ExpiresAt: time.Now().Add(time.Hour).Unix(),
		Audience:  aud,
		ID:        tokenID.String(),
	}
	now := time.Now()
	monkey.Patch(time.Now, func() time.Time {
		return now.Add(time.Hour + time.Second)
	})
	strtoken, err := token.Sign([]byte(key))
	if err != nil {
		t.Fatalf(err.Error())
	}
	_, err = VerifyToken(strtoken, []byte(key))
	if err == nil {
		t.Fatalf("unexpected pass in token verify")
	}
}
