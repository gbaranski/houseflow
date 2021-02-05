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
	"github.com/gbaranski/houseflow/pkg/token"
	"github.com/gbaranski/houseflow/pkg/utils"
)

func TestSyncWithoutToken(t *testing.T) {
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
	req, _ := http.NewRequest(http.MethodPost, "/webhook", bytes.NewReader(benc))
	req.Header.Add("Content-Type", "application/json")
	f.Router.ServeHTTP(w, req)

	if w.Code != http.StatusBadRequest {
		t.Fatalf("unexpected sync response %d", w.Code)
	}

}

func TestSyncNoDevices(t *testing.T) {
	token := token.Parsed{
		ExpiresAt: uint32(time.Now().Add(time.Hour).Unix()),
		Audience:  []byte(realUser.ID),
	}
	signedToken, err := token.Sign([]byte(opts.AccessKey))
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
	req, _ := http.NewRequest(http.MethodPost, "/webhook", bytes.NewReader(benc))
	req.Header.Add("Content-Type", "application/json")
	req.Header.Add("Authorization", fmt.Sprintf("Bearer %s", signedToken.Base64()))
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
	tdevice := devices[0]
	userDevices = append(userDevices, userDevice{
		UserID:   realUser.ID,
		DeviceID: tdevice.ID,
		Read:     true,
		Write:    false,
		Execute:  false,
	})
	defer func() {
		// Clear the slice
		userDevices = make([]userDevice, 0)
	}()
	token := token.Parsed{
		ExpiresAt: uint32(time.Now().Add(time.Hour).Unix()),
		Audience:  []byte(realUser.ID),
	}
	signedToken, err := token.Sign([]byte(opts.AccessKey))
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
	req, _ := http.NewRequest(http.MethodPost, "/webhook", bytes.NewReader(benc))
	req.Header.Add("Content-Type", "application/json")
	req.Header.Add("Authorization", fmt.Sprintf("Bearer %s", signedToken.Base64()))
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
	if pdevice.ID != tdevice.ID {
		t.Fatalf("device ids doesn't match, expected: %s, received: %s", tdevice.ID, pdevice.ID)

	}
}
