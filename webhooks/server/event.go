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

func handleMessagePublish(e *types.MessageEvent, client *services.FirebaseClient) {
	clientDataArray, err := services.GetClientData(e.FromClientID)
	if err != nil {
		log.Printf("Error occured when retreiving client data %s\n", err)
		return
	}

	requestClientData := clientDataArray.Data[0]
	requestDeviceUID := e.Topic[:uuidLength]

	preTrimmedRequest := strings.TrimPrefix(e.Topic, requestDeviceUID+"/event/")
	trimmedRequest := strings.TrimSuffix(preTrimmedRequest, "/request")

	firebaseUserUsername, err := services.GetUserUsername(context.Background(), client, requestClientData.Username)
	if err != nil {
		log.Printf("Error occured when retreiving firebase user username %s\n", err)
		return
	}
	deviceRequest := types.DeviceRequest{IPAddress: requestClientData.IPAddress, Request: trimmedRequest, Timestamp: e.Timestamp, Username: firebaseUserUsername}
	firebaseDevice, err := services.GetFirebaseDevice(context.Background(), client, requestDeviceUID)
	if err != nil {
		log.Printf("Error occured when retreiving firebase device %s\n", err)
		return
	}

	id, err := services.AddDeviceHistory(context.Background(), client, firebaseDevice, &deviceRequest)

	if err != nil {
		log.Printf("Error occured when adding device history %s\n", err)
		return
	}
	log.Printf("Sucessfully added request history %s\n", id)
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
	case "message_publish":
		e, err := utils.BytesToMessageEvent(bytes)
		if err != nil {
			log.Printf("Failed parsing bytes to MessageEvent %s\n", err)
			return
		}
		if strings.HasPrefix(e.FromClientID, "device_") == true || strings.HasSuffix(e.Topic, "/request") == false {
			return
		}
		handleMessagePublish(e, client)
	default:
		log.Printf("Unrecognized action %s\n", e.Action)
	}
}
