package parse

import (
	"encoding/json"
	"io"
	"io/ioutil"

	types "github.com/gbaranski/houseflow/webhooks/types"
)

// BodyToBytes parses request body to bytes array
func BodyToBytes(reqBody io.ReadCloser) ([]byte, error) {
	body, readErr := ioutil.ReadAll(reqBody)
	return body, readErr
}

// Those 3 functions are bad, it could be rewritten to one, but golang doesn't have generics yet

// BytesToWebhookEvent parses bytes array to WebhookEvent
func BytesToWebhookEvent(byteArr []byte) (*types.WebhookEvent, error) {
	e := new(types.WebhookEvent)
	jsonErr := json.Unmarshal(byteArr, e)
	return e, jsonErr
}

// BytesToConnectedEvent parses bytes array to ConnectedEvent
func BytesToConnectedEvent(byteArr []byte) (*types.ConnectedEvent, error) {
	e := new(types.ConnectedEvent)
	jsonErr := json.Unmarshal(byteArr, e)
	return e, jsonErr
}

// BytesToDisconnectedEvent parses bytes array to DisconnectedEvent
func BytesToDisconnectedEvent(byteArr []byte) (*types.DisconnectedEvent, error) {
	e := new(types.DisconnectedEvent)
	jsonErr := json.Unmarshal(byteArr, e)
	return e, jsonErr
}

// BytesToMessageEvent parses bytes array to MessageEvent
func BytesToMessageEvent(byteArr []byte) (*types.MessageEvent, error) {
	e := new(types.MessageEvent)
	jsonErr := json.Unmarshal(byteArr, e)
	return e, jsonErr
}

// BytesToClientData parses body to GetClientResponse
func BytesToClientData(byteArr []byte) (*types.GetClientResponse, error) {
	e := new(types.GetClientResponse)
	jsonErr := json.Unmarshal(byteArr, e)
	return e, jsonErr

}
