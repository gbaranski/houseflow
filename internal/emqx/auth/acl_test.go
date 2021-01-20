package auth

import (
	"bytes"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/google/uuid"
)

func TestACLAsDevice(t *testing.T) {
	deviceID := uuid.New()
	p := ACLRequest{
		Access:   1,
		Username: base64.StdEncoding.EncodeToString(devicePublicKey),
		ClientID: deviceID.String(),
		IP:       "80.21.12.18",
		Topic:    fmt.Sprintf("%s/command/something", deviceID.String()),
	}
	pjson, err := json.Marshal(p)
	if err != nil {
		t.Fatalf("fail encode payload %s", err.Error())
	}
	w := httptest.NewRecorder()
	req, _ := http.NewRequest(http.MethodPost, "/acl", bytes.NewReader(pjson))
	req.Header.Add("Content-Type", "application/json")
	a.Router.ServeHTTP(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("unexpected response code %d, expected %d", w.Code, http.StatusOK)
	}
}

func TestACLAsFakeDevice(t *testing.T) {
	fakeDeviceID := uuid.New()
	targetDeviceID := uuid.New()
	p := ACLRequest{
		Access:   1,
		Username: base64.StdEncoding.EncodeToString(devicePublicKey),
		ClientID: fakeDeviceID.String(),
		IP:       "80.21.12.18",
		Topic:    fmt.Sprintf("%s/command/something", targetDeviceID.String()),
	}
	pjson, err := json.Marshal(p)
	if err != nil {
		t.Fatalf("fail encode payload %s", err.Error())
	}
	w := httptest.NewRecorder()
	req, _ := http.NewRequest(http.MethodPost, "/acl", bytes.NewReader(pjson))
	req.Header.Add("Content-Type", "application/json")
	a.Router.ServeHTTP(w, req)

	if w.Code != http.StatusUnauthorized {
		t.Fatalf("unexpected response code %d, expected %d", w.Code, http.StatusOK)
	}

}

func TestACLAsService(t *testing.T) {
	p := ACLRequest{
		Access:   1,
		Username: base64.StdEncoding.EncodeToString(serverPublicKey),
		ClientID: "some-service",
		IP:       "80.21.12.18",
		Topic:    "random-topic/djsajadsd/dsajdsajads",
	}
	pjson, err := json.Marshal(p)
	if err != nil {
		t.Fatalf("fail encode payload %s", err.Error())
	}
	w := httptest.NewRecorder()
	req, _ := http.NewRequest(http.MethodPost, "/acl", bytes.NewReader(pjson))
	req.Header.Add("Content-Type", "application/json")
	a.Router.ServeHTTP(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("unexpected response code %d, expected %d", w.Code, http.StatusOK)
	}
}

func TestACLAsFakeService(t *testing.T) {
	p := ACLRequest{
		Access:   1,
		Username: base64.StdEncoding.EncodeToString(devicePublicKey),
		ClientID: "some-service",
		IP:       "80.21.12.18",
		Topic:    "random-topic/djsajadsd/dsajdsajads",
	}
	pjson, err := json.Marshal(p)
	if err != nil {
		t.Fatalf("fail encode payload %s", err.Error())
	}
	w := httptest.NewRecorder()
	req, _ := http.NewRequest(http.MethodPost, "/acl", bytes.NewReader(pjson))
	req.Header.Add("Content-Type", "application/json")
	a.Router.ServeHTTP(w, req)

	if w.Code != http.StatusUnauthorized {
		t.Fatalf("unexpected response code %d, expected %d", w.Code, http.StatusOK)
	}
}
