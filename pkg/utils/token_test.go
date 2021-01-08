package utils

import (
	"fmt"
	"testing"
	"time"
)

func TestValidCreateToken(t *testing.T) {
	key := GenerateRandomString(20)
	aud := GenerateRandomString(20)
	token, err := CreateJWTToken(TokenOptions{
		Duration: time.Hour,
		Key:      key,
		Audience: aud,
	})
	if err != nil {
		t.Fatalf(err.Error())
	}
	td, err := VerifyToken(token.Token.Raw, []byte(key))
	if err != nil {
		t.Fatalf("fail verify token: %s", err.Error())
	}
	if td.Claims.Valid() != nil {
		t.Fatalf("invalid claims: %s", td.Claims.Valid().Error())
	}
	if td.Claims.Audience != aud {
		t.Fatalf("audience doesn't match, expected: %s, received: %s", aud, token.Claims.Audience)
	}
	exp := time.Unix(td.Claims.ExpiresAt, 0)
	now := time.Now()
	if exp.Sub(now).Minutes() < 59 {
		t.Fatalf("too big difference, expected: >= 59, received: %f", exp.Sub(now).Minutes())
	}
}

func TestExpiredCreateToken(t *testing.T) {
	key := GenerateRandomString(20)
	aud := GenerateRandomString(20)
	token, err := CreateJWTToken(TokenOptions{
		Duration: time.Millisecond, // not zero
		Key:      key,
		Audience: aud,
	})
	if err != nil {
		t.Fatalf(err.Error())
	}
	fmt.Println(token.Token.Raw)
	time.Sleep(time.Millisecond * 5)
	td, err := VerifyToken(token.Token.Raw, []byte(key))
	if err != nil {
		t.Fatalf("fail verify token: %s", err.Error())
	}
	if td.Claims.Valid() != nil {
		t.Fatalf("invalid claims: %s", td.Claims.Valid().Error())
	}
	if td.Claims.Audience != aud {
		t.Fatalf("audience doesn't match, expected: %s, received: %s", aud, token.Claims.Audience)
	}
	exp := time.Unix(td.Claims.ExpiresAt, 0)
	now := time.Now()
	if exp.Sub(now).Minutes() < 59 {
		t.Fatalf("too big difference, expected: >= 59, received: %f", exp.Sub(now).Minutes())
	}
}
