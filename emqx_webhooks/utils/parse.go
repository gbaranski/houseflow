package parse

import (
	"encoding/json"

	types "github.com/gbaranski/houseflow/webhooks/types"
)

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
