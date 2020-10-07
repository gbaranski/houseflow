package event

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"strings"

	services "github.com/gbaranski/homeflow/webhooks/services"
	types "github.com/gbaranski/homeflow/webhooks/types"
	utils "github.com/gbaranski/homeflow/webhooks/utils"
)

func handleConnected(e *types.ConnectedEvent, client *services.FirebaseClient) {
	log.Printf("Client connected: %s\n", e.ClientID)
	services.UpdateDeviceStatus(context.Background(), client, e.Username, true)
}

func handleDisconnected(e *types.DisconnectedEvent, client *services.FirebaseClient) {
	log.Printf("Client disconnected: %s\n", e.ClientID)
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

	log.Printf("Received request at /event, action: %s\n", e.Action)

	if strings.HasPrefix(e.ClientID, "device_") == false {
		log.Printf("Cancelling %s because it isn't device", e.ClientID)
		return
	}

	switch e.Action {
	case "client_connected":
		e, err := utils.BytesToConnectedEvent(bytes)
		if err != nil {
			log.Printf("Failed parsing bytes to ConnectedEvent %s\n", err)
			return
		}
		handleConnected(e, client)
	case "client_disconnected":
		e, err := utils.BytesToDisconnectedEvent(bytes)
		if err != nil {
			log.Printf("Failed parsing bytes to DisconnectedEvent %s\n", err)
			return
		}
		handleDisconnected(e, client)
	default:
		log.Printf("Unrecognized action %s\n", e.Action)
	}
}
