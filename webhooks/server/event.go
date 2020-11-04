package event

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"strings"

	services "github.com/gbaranski/houseflow/webhooks/services"
	types "github.com/gbaranski/houseflow/webhooks/types"
	utils "github.com/gbaranski/houseflow/webhooks/utils"
)

const uuidLength = 36

func handleConnected(e *types.ConnectedEvent, client *services.FirebaseClient) {
	services.UpdateDeviceStatus(context.Background(), client, e.Username, true)
}

func handleDisconnected(e *types.DisconnectedEvent, client *services.FirebaseClient) {
	services.UpdateDeviceStatus(context.Background(), client, e.Username, false)
}

// OnEvent handles /event request
func OnEvent(w http.ResponseWriter, req *http.Request, client *services.FirebaseClient) {
	fmt.Fprintf(w, "Hello at /event")

	bytes, err := utils.BodyToBytes(req.Body)
	if err != nil {
		log.Printf("Failed parsing request body %s\n", err)
		return
	}
	e, err := utils.BytesToWebhookEvent(bytes)
	if err != nil {
		log.Printf("Failed parsing bytes to WebhookEvent %s\n", err)
		return
	}

	switch e.Action {
	case "client_connected":
		e, err := utils.BytesToConnectedEvent(bytes)
		if err != nil {
			log.Printf("Failed parsing bytes to ConnectedEvent %s\n", err)
			return
		}
		if strings.HasPrefix(e.ClientID, "device_") == false {
			return
		}

		handleConnected(e, client)
	case "client_disconnected":
		e, err := utils.BytesToDisconnectedEvent(bytes)
		if err != nil {
			log.Printf("Failed parsing bytes to DisconnectedEvent %s\n", err)
			return
		}
		if strings.HasPrefix(e.ClientID, "device_") == false {
			return
		}
		handleDisconnected(e, client)
	default:
		log.Printf("Unrecognized action %s\n", e.Action)
	}
}
