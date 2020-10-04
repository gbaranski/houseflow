package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
)

type webhookEvent struct {
	Username string `json:"username"`
	Action   string `json:"action"`
	ClientID string `json:"clientid"`
}

type connectedEvent struct {
	webhookEvent
	ProtoVersion       int    `json:"proto_ver"`
	KeepAlive          int    `json:"keepalive"`
	IPAddress          string `json:"ipaddress"`
	ConnectedTimestamp uint64 `json:"connected_at"`
}

type disconnectedEvent struct {
	webhookEvent
	Reason string `json:"reason"`
}

func handleConnected(e *connectedEvent) {
	fmt.Printf("Client connected: %s\n", e.ClientID)
}

func handleDisconnected(e *disconnectedEvent) {
	fmt.Printf("Client disconnected: %s\n", e.ClientID)
}

func onEvent(w http.ResponseWriter, req *http.Request) {
	e := new(webhookEvent)
	body, readErr := ioutil.ReadAll(req.Body)
	if readErr != nil {
		log.Println(readErr)
		return
	}
	log.Println(string(body))

	jsonErr := json.Unmarshal(body, e)
	if jsonErr != nil {
		log.Println(jsonErr)
		return
	}

	fmt.Fprintf(w, "Hello at /event")
	fmt.Println("Received request at /event")
	fmt.Printf("Action: %s\n", e.Action)

	switch e.Action {
	case "client_connected":
		e := new(connectedEvent)
		jsonErr := json.Unmarshal(body, e)
		if jsonErr != nil {
			log.Println(jsonErr)
			return
		}
		handleConnected(e)
	case "client_disconnected":
		e := new(disconnectedEvent)
		jsonErr := json.Unmarshal(body, e)
		if jsonErr != nil {
			log.Println(jsonErr)
			return
		}
		handleDisconnected(e)
	default:
		log.Printf("Unrecognized action %s\n", e.Action)
	}
}

func main() {
	fmt.Println("Hello, world")
	http.HandleFunc("/event", onEvent)

	http.ListenAndServe(":8001", nil)
}
