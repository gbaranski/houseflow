package fulfillment

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	ftypes "github.com/gbaranski/houseflow/pkg/fulfillment"
	"github.com/gbaranski/houseflow/pkg/utils"
)

func TestSyncUnauthorized(t *testing.T) {
	body := ftypes.SyncRequest{
		RequestID: utils.GenerateRandomString(10),
		Inputs: []ftypes.SyncRequestInput{
			{
				Intent: "action.devices.SYNC",
			},
		},
	}

	benc, err := json.Marshal(body)
	if err != nil {
		panic(err)
	}

	w := httptest.NewRecorder()
	req, _ := http.NewRequest(http.MethodPost, "/fulfillment", bytes.NewReader(benc))
	req.Header.Add("Content-Type", "application/json")
	f.Router.ServeHTTP(w, req)

	if w.Code != http.StatusUnauthorized {
		t.Fatalf("unexpected sync response %d", w.Code)
	}

}

func TestSyncNoDevices(t *testing.T) {
	token := utils.Token{
		Audience:  realUser.ID.Hex(),
		ExpiresAt: time.Now().Add(time.Hour).Unix(),
	}
	strtoken, err := token.Sign([]byte(opts.AccessKey))
	if err != nil {
		t.Fatalf("fail when signing token %s", err.Error())
	}
	body := ftypes.SyncRequest{
		RequestID: utils.GenerateRandomString(10),
		Inputs: []ftypes.SyncRequestInput{
			{
				Intent: "action.devices.SYNC",
			},
		},
	}
	benc, err := json.Marshal(body)
	if err != nil {
		panic(err)
	}

	w := httptest.NewRecorder()
	req, _ := http.NewRequest(http.MethodPost, "/fulfillment", bytes.NewReader(benc))
	req.Header.Add("Content-Type", "application/json")
	req.Header.Add("Authorization", fmt.Sprintf("Bearer %s", strtoken))
	f.Router.ServeHTTP(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("unexpected sync response %d", w.Code)
	}
	var res ftypes.SyncResponse
	err = json.Unmarshal(w.Body.Bytes(), &res)
	if err != nil {
		t.Fatalf("fail unmarshall sync response %s", err.Error())
	}
	if body.RequestID != res.RequestID {
		t.Fatalf("requestID doesn't match, expected %s, received %s", body.RequestID, res.RequestID)
	}
	if len(res.Payload.Devices) > 0 {
		t.Fatalf("not expected any devices from response, received %d devices", len(res.Payload.Devices))
	}
}

func TestSync(t *testing.T) {
	device := db.Devices[0]
	realUser.Devices = append(realUser.Devices, device.ID.Hex())
	defer func() {
		// Clear the slice
		realUser.Devices = make([]string, 0)
	}()
	token := utils.Token{
		Audience:  realUser.ID.Hex(),
		ExpiresAt: time.Now().Add(time.Hour).Unix(),
	}
	strtoken, err := token.Sign([]byte(opts.AccessKey))
	if err != nil {
		t.Fatalf("fail when signing token %s", err.Error())
	}
	body := ftypes.SyncRequest{
		RequestID: utils.GenerateRandomString(10),
		Inputs: []ftypes.SyncRequestInput{
			{
				Intent: "action.devices.SYNC",
			},
		},
	}
	benc, err := json.Marshal(body)
	if err != nil {
		panic(err)
	}

	w := httptest.NewRecorder()
	req, _ := http.NewRequest(http.MethodPost, "/fulfillment", bytes.NewReader(benc))
	req.Header.Add("Content-Type", "application/json")
	req.Header.Add("Authorization", fmt.Sprintf("Bearer %s", strtoken))
	f.Router.ServeHTTP(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("unexpected sync response %d", w.Code)
	}
	var res ftypes.SyncResponse
	err = json.Unmarshal(w.Body.Bytes(), &res)
	if err != nil {
		t.Fatalf("fail unmarshall sync response %s", err.Error())
	}
	if body.RequestID != res.RequestID {
		t.Fatalf("requestID doesn't match, expected %s, received %s", body.RequestID, res.RequestID)
	}
	if len(res.Payload.Devices) != 1 {
		t.Fatalf("expected one from response, received %d devices", len(res.Payload.Devices))
	}
	pdevice := res.Payload.Devices[0]
	if pdevice.ID != device.ID.Hex() {
		t.Fatalf("device ids doesn't match, expected: %s, received: %s", device.ID, pdevice.ID)

	}
}
