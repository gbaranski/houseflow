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

func TestExecute(t *testing.T) {
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
		Execute:  true,
	})
	defer func() {
		// Clear the slice
		userDevices = make([]userDevice, 0)
	}()
	body := ftypes.ExecuteRequest{
		RequestID: utils.GenerateRandomString(10),
		Inputs: []ftypes.ExecuteRequestInput{
			{
				Intent: "action.devices.EXECUTE",
				Payload: ftypes.ExecuteRequestPayload{
					Commands: []ftypes.ExecuteRequestCommands{
						{
							Devices: []ftypes.QueryRequestPayloadDevice{
								{
									ID: tdevice.ID, // with access
								},
								{
									ID: devices[1].ID, // without acesss
								},
							},
							Execution: []ftypes.ExecuteRequestExecution{
								{
									Command: "action.devices.commands.OnOff",
									Params: map[string]interface{}{
										"on": true,
									},
								},
							},
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
	var res ftypes.ExecuteResponse
	err = json.Unmarshal(w.Body.Bytes(), &res)
	if err != nil {
		t.Fatalf("fail unmarshall query response %s", err.Error())
	}
	if res.RequestID != body.RequestID {
		t.Fatalf("requestID doesn't match")
	}
	if res.Payload.ErrorCode != "" {
		t.Fatalf("non empty errorCode: %s, debugstr: %s", res.Payload.ErrorCode, res.Payload.DebugString)
	}

	select {
	case cmd := <-commands:
		if cmd != body.Inputs[0].Payload.Commands[0].Execution[0].Command {
			t.Fatalf("fail command not equal, expected: %s, received: %s", cmd, body.Inputs[0].Payload.Commands[0].Execution[0].Command)
		}
	case <-time.After(1 * time.Second):
		t.Fatalf("timeout waiting for msg")
	}

	for _, v := range res.Payload.Commands {
		for _, id := range v.IDs {
			if id == tdevice.ID {
				if v.Status != "SUCCESS" {
					t.Fatalf("unexpected status, expected %s, received %s", "SUCCESS", v.Status)
				}
			} else {
				if v.Status != "ERROR" {
					t.Fatalf("unexpected status, expected %s, received %s", "ERROR", v.Status)
				}
				if v.ErrorCode != "authFailure" {
					t.Fatalf("unexpected errorCode, expected %s, received %s", "authFailure", v.ErrorCode)
				}
			}
		}

	}
}
