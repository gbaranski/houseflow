package auth

import (
	"bytes"
	"crypto/ed25519"
	"crypto/rand"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestConnectAsService(t *testing.T) {
	p := ConnectRequest{
		ClientID: "some-service",
		IP:       "80.21.12.18",
		Username: base64.StdEncoding.EncodeToString(pkey),
		Password: base64.StdEncoding.EncodeToString(ed25519.Sign(skey, pkey)),
	}
	pjson, err := json.Marshal(p)
	if err != nil {
		t.Fatalf("fail encode payload %s", err.Error())
	}
	w := httptest.NewRecorder()
	req, _ := http.NewRequest(http.MethodPost, "/user", bytes.NewReader(pjson))
	req.Header.Add("Content-Type", "application/json")
	a.Router.ServeHTTP(w, req)

	fmt.Println(w.Body.String())
	if w.Code != http.StatusOK {
		t.Fatalf("unexpected response code %d, expected %d", w.Code, http.StatusOK)
	}
}

func TestConnectAsFakeService(t *testing.T) {
	_, randomskey, err := ed25519.GenerateKey(rand.Reader)
	if err != nil {
		t.Fatalf("failed generating ed25519 key %s", err.Error())
	}

	p := ConnectRequest{
		ClientID: "some-service",
		IP:       "80.21.12.18",
		Username: base64.StdEncoding.EncodeToString(pkey),
		Password: base64.StdEncoding.EncodeToString(ed25519.Sign(randomskey, pkey)),
	}
	pjson, err := json.Marshal(p)
	if err != nil {
		t.Fatalf("fail encode payload %s", err.Error())
	}
	w := httptest.NewRecorder()
	req, _ := http.NewRequest(http.MethodPost, "/user", bytes.NewReader(pjson))
	req.Header.Add("Content-Type", "application/json")
	a.Router.ServeHTTP(w, req)

	fmt.Println(w.Body.String())
	if w.Code != http.StatusUnauthorized {
		t.Fatalf("unexpected response code %d, expected %d", w.Code, http.StatusOK)
	}
}
