package utils

import (
	"encoding/base64"
	"fmt"
	"net/http"
	"os"
	"strings"
	"testing"

	"github.com/gbaranski/houseflow/pkg/token"
	"golang.org/x/crypto/bcrypt"
)

func TestGenerateRandomString(t *testing.T) {
	for i := 10; i < 100; i++ {
		var strings [10]string
		for range strings {
			random := GenerateRandomString(i)
			if len(random) != i {
				t.Fatalf("random string length doesn't match, expected: %d, received: %d\n", i, len(random))
			}
			for _, v := range strings {
				if v == random {
					t.Fatalf("no enough randomness at length %d", i)
				}
			}
		}
	}
}

func TestParseSignedPayload(t *testing.T) {
	type Payload struct {
		sig    string
		encsig string
		msg    string
	}

	var payloads [100]Payload
	for i := 0; i < 100; i++ {
		sig := GenerateRandomString(32)
		encSig := base64.StdEncoding.EncodeToString([]byte(sig))
		payloads[i] = Payload{
			sig:    sig,
			encsig: encSig,
			msg:    GenerateRandomString(100),
		}
	}

	for _, p := range payloads {
		fp := strings.Join([]string{p.encsig, p.msg}, ".")
		msg, sig, err := ParseSignedPayload([]byte(fp))
		if err != nil {
			t.Fatalf("fail err: %s", err.Error())
		}
		if msg != p.msg {
			t.Fatalf("fail msg doesn't match, received: %s, expected: %s", msg, p.msg)
		}
		if string(sig) != p.sig {
			t.Fatalf("fail sig doesn't match, received: %s, expected: %s", msg, p.msg)
		}
	}

	// Test some constant
	msg, sig, err := ParseSignedPayload([]byte("aGVsbG8=.world"))
	if err != nil {
		t.Fatalf("fail constant test with err: %s", err.Error())
	}
	if msg != "world" {
		t.Fatalf("fail constant msg test, received: %s, expected: world", msg)
	}
	if string(sig) != "hello" {
		t.Fatalf("fail constant sig test, received: %s, expected: hello", sig)
	}

	_, _, err = ParseSignedPayload([]byte("thisshouldthrow"))
	if err == nil {
		t.Fatalf("expected to return error")
	}
}

func TestExtractHeaderToken(t *testing.T) {
	tok := GenerateRandomString(64)
	req := http.Request{Header: http.Header{}}
	req.Header.Set("Authorization", fmt.Sprintf("Bearer %s", tok))
	str := token.ExtractHeaderToken(&req)
	if str == nil {
		t.Fatalf("unexpected nil token")
	}
	if *str != tok {
		t.Fatalf("tokens doesn't match, received: %s, expected: %s", *str, tok)
	}

	req.Header.Del("Authorization")
	str = token.ExtractHeaderToken(&req)
	if str != nil {
		t.Fatalf("expected token to be nil")
	}

}

func TestHashPassword(t *testing.T) {
	for i := 0; i < 2; i++ {
		pass := GenerateRandomString(16)
		hash, err := HashPassword([]byte(pass))
		if err != nil {
			t.Fatalf(err.Error())
		}
		err = bcrypt.CompareHashAndPassword(hash, []byte(pass))
		if err != nil {
			t.Fatalf("err while comparing: %s", err.Error())
		}
	}

	// should fail
	hash, err := HashPassword([]byte("helloworld"))
	if err != nil {
		t.Fatalf(err.Error())
	}
	err = bcrypt.CompareHashAndPassword(hash, []byte("worldhello"))
	if err == nil {
		t.Fatalf("expected error for wrong password, err returned nil")
	}
}

func TestCompareHash(t *testing.T) {
	pass := GenerateRandomString(20)
	hash, err := bcrypt.GenerateFromPassword([]byte(pass), bcrypt.DefaultCost)
	if err != nil {
		t.Fatalf(err.Error())
	}
	match := ComparePasswordAndHash(pass, hash)
	if !match {
		t.Fatalf("fail hash doesn't match")
	}
}

func TestCompareHashInvalid(t *testing.T) {
	pass := []byte(GenerateRandomString(20))
	hash, err := bcrypt.GenerateFromPassword(pass, bcrypt.DefaultCost)
	if err != nil {
		t.Fatalf(err.Error())
	}
	match := ComparePasswordAndHash(GenerateRandomString(20), hash)
	if match {
		t.Fatalf("unexpected hash match")
	}
}

func TestMustGetEnv(t *testing.T) {
	os.Setenv("testingvar", "helloworld")
	defer func() {
		os.Unsetenv("testingvar")
	}()

	env := MustGetEnv("testingvar")
	if env != "helloworld" {
		t.Fatalf("environment variable doesnt match")
	}
}

func TestMustGetEnvNotPresent(t *testing.T) {
	defer func() {
		if r := recover(); r == nil {
			t.Fatalf("function expected to panic")
		}
	}()

	MustGetEnv("skfjafasdkjfsdkljafdskljafdsklajbgvbjkfdsjkgjhkfdasjhk")
}
