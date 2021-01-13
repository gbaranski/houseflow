package auth

import (
	"bytes"
	"crypto/ed25519"
	"crypto/rand"
	"encoding/base64"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/gbaranski/houseflow/pkg/types"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

func TestConnectAsService(t *testing.T) {
	p := ConnectRequest{
		ClientID: "some-service",
		IP:       "80.21.12.18",
		Username: base64.StdEncoding.EncodeToString(serverPublicKey),
		Password: base64.StdEncoding.EncodeToString(ed25519.Sign(serverPrivateKey, serverPublicKey)),
	}
	pjson, err := json.Marshal(p)
	if err != nil {
		t.Fatalf("fail encode payload %s", err.Error())
	}
	w := httptest.NewRecorder()
	req, _ := http.NewRequest(http.MethodPost, "/user", bytes.NewReader(pjson))
	req.Header.Add("Content-Type", "application/json")
	a.Router.ServeHTTP(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("unexpected response code %d, expected %d", w.Code, http.StatusOK)
	}
}

func TestConnectAsFakeService(t *testing.T) {
	_, randomPrivateKey, err := ed25519.GenerateKey(rand.Reader)
	if err != nil {
		t.Fatalf("failed generating ed25519 key %s", err.Error())
	}

	p := ConnectRequest{
		ClientID: "some-service",
		IP:       "80.21.12.18",
		Username: base64.StdEncoding.EncodeToString(serverPublicKey),
		Password: base64.StdEncoding.EncodeToString(ed25519.Sign(randomPrivateKey, serverPublicKey)),
	}
	pjson, err := json.Marshal(p)
	if err != nil {
		t.Fatalf("fail encode payload %s", err.Error())
	}
	w := httptest.NewRecorder()
	req, _ := http.NewRequest(http.MethodPost, "/user", bytes.NewReader(pjson))
	req.Header.Add("Content-Type", "application/json")
	a.Router.ServeHTTP(w, req)

	if w.Code != http.StatusUnauthorized {
		t.Fatalf("unexpected response code %d, expected %d", w.Code, http.StatusOK)
	}
}

func TestConnectAsDevice(t *testing.T) {
	device := types.Device{
		ID:        primitive.NewObjectID(),
		PublicKey: base64.StdEncoding.EncodeToString(devicePublicKey),
	}
	devices = append(devices, device)
	defer func() {
		devices = make([]types.Device, 0)
	}()

	p := ConnectRequest{
		ClientID: device.ID.Hex(),
		IP:       "80.21.12.18",
		Username: base64.StdEncoding.EncodeToString(devicePublicKey),
		Password: base64.StdEncoding.EncodeToString(ed25519.Sign(devicePrivateKey, devicePublicKey)),
	}
	pjson, err := json.Marshal(p)
	if err != nil {
		t.Fatalf("fail encode payload %s", err.Error())
	}
	w := httptest.NewRecorder()
	req, _ := http.NewRequest(http.MethodPost, "/user", bytes.NewReader(pjson))
	req.Header.Add("Content-Type", "application/json")
	a.Router.ServeHTTP(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("unexpected response code %d, expected %d", w.Code, http.StatusOK)
	}
}
