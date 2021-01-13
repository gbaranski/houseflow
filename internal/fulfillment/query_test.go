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

func TestQuery(t *testing.T) {
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
	body := ftypes.QueryRequest{
		RequestID: utils.GenerateRandomString(10),
		Inputs: []ftypes.QueryRequestInput{
			{
				Intent: "action.devices.QUERY",
				Payload: ftypes.QueryRequestPayload{
					Devices: []ftypes.QueryRequestPayloadDevice{
						{
							ID: tdevice.ID.Hex(), // User has access
						},
						{
							ID: db.Devices[1].ID.Hex(), // User doesn't have access
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
	var res ftypes.QueryResponse
	err = json.Unmarshal(w.Body.Bytes(), &res)
	if err != nil {
		t.Fatalf("fail unmarshall query response %s", err.Error())
	}
	if res.RequestID != body.RequestID {
		t.Fatalf("requestID doesn't match")
	}
	for k, v := range res.Payload.Devices {
		obj := v.(map[string]interface{})

		if k == tdevice.ID.Hex() {
			if obj["status"] != "SUCCESS" {
				t.Fatalf("unexpected status, expected %s, received %s", "SUCCESS", obj["status"])
			}
		} else {
			if obj["status"] != "ERROR" {
				t.Fatalf("unexpected status, expected %s, received %s", "ERROR", obj["status"])
			}
			if obj["errorCode"] != "relinkRequired" {
				t.Fatalf("unexpected errorCode, expected %s, received %s", "relinkRequired", obj["status"])
			}
		}

	}
	fmt.Println(res.Payload.Devices)
}
