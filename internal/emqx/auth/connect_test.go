package auth

import (
	"bytes"
	"crypto/ed25519"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestConnectAsService(t *testing.T) {
	username := base64.StdEncoding.EncodeToString(pkey)
	sig := ed25519.Sign(skey, pkey)
	sigEnc := base64.StdEncoding.EncodeToString(sig)

	p := ConnectRequest{
		ClientID: "some-service",
		IP:       "80.21.12.18",
		Username: username,
		Password: sigEnc,
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
