package utils

import (
	"encoding/base64"
	"strings"
	"testing"
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

type Payload struct {
	sig    string
	encsig string
	msg    string
}

func TestParseSignedPayload(t *testing.T) {
	// Single constant
	payloads := []Payload{
		{
			sig:    "helloworld",
			encsig: "aGVsbG93b3JsZA==",
			msg:    "houseflowisawesome",
		},
	}
	for i := 0; i < 100; i++ {
		sig := GenerateRandomString(32)
		encSig := base64.StdEncoding.EncodeToString([]byte(sig))
		payloads = append(payloads, Payload{
			sig:    sig,
			encsig: encSig,
			msg:    GenerateRandomString(100),
		})
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
}
