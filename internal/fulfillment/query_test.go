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

func TestQuery(t *testing.T) {
	token := token.Parsed{
		ExpiresAt: uint32(time.Now().Add(time.Hour).Unix()),
	}
	copy(token.Audience[:], realUser.ID)
	signedToken, err := token.Sign([]byte(opts.AccessKey))
	if err != nil {
		t.Fatalf("fail when signing token %s", err.Error())
	}

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
	body := ftypes.QueryRequest{
		RequestID: utils.GenerateRandomString(10),
		Inputs: []ftypes.QueryRequestInput{
			{
				Intent: "action.devices.QUERY",
				Payload: ftypes.QueryRequestPayload{
					Devices: []ftypes.QueryRequestPayloadDevice{
						{
							ID: tdevice.ID, // User has access
						},
						{
							ID: devices[1].ID, // User doesn't have access
						},
					},
				},
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
		t.Fatalf("unexpected query response %d", w.Code)
	}
	var res ftypes.QueryResponse
	err = json.Unmarshal(w.Body.Bytes(), &res)
	if err != nil {
		t.Fatalf("fail unmarshall query response %s", err.Error())
	}
	if res.RequestID != body.RequestID {
		t.Fatalf("requestID doesn't match")
	}
	if res.Payload.ErrorCode != "" {
		t.Fatalf("non empty errorcode: %s, debugstr: %s", res.Payload.DebugString, res.Payload.ErrorCode)
	}
	for k, v := range res.Payload.Devices {
		obj := v.(map[string]interface{})

		if k == tdevice.ID {
			if obj["status"] != "SUCCESS" {
				t.Fatalf("unexpected status, expected %s, received %s", "SUCCESS", obj["status"])
			}
		} else {
			if obj["status"] != "ERROR" {
				t.Fatalf("unexpected status, expected %s, received %s", "ERROR", obj["status"])
			}
			if obj["errorCode"] != "authFailure" {
				t.Fatalf("unexpected errorCode, expected %s, received %s", "authFailure", obj["status"])
			}
		}

	}
}
