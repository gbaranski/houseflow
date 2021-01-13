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

func TestExecute(t *testing.T) {
	token := utils.Token{
		Audience:  realUser.ID.Hex(),
		ExpiresAt: time.Now().Add(time.Hour).Unix(),
	}
	strtoken, err := token.Sign([]byte(opts.AccessKey))
	if err != nil {
		t.Fatalf("fail when signing token %s", err.Error())
	}

	tdevice := db.Devices[0]
	realUser.Devices = append(realUser.Devices, tdevice.ID.Hex())
	defer func() {
		// Clear the slice
		realUser.Devices = make([]string, 0)
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
									ID: tdevice.ID.Hex(), // with access
								},
								{
									ID: db.Devices[1].ID.Hex(), // without acesss
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
	req, _ := http.NewRequest(http.MethodPost, "/fulfillment", bytes.NewReader(benc))
	req.Header.Add("Content-Type", "application/json")
	req.Header.Add("Authorization", fmt.Sprintf("Bearer %s", strtoken))
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
			if id == tdevice.ID.Hex() {
				if v.Status != "SUCCESS" {
					t.Fatalf("unexpected status, expected %s, received %s", "SUCCESS", v.Status)
				}
			} else {
				if v.Status != "ERROR" {
					t.Fatalf("unexpected status, expected %s, received %s", "ERROR", v.Status)
				}
				if v.ErrorCode != "relinkRequired" {
					t.Fatalf("unexpected errorCode, expected %s, received %s", "relinkRequired", v.Status)
				}
			}
		}

	}
}
