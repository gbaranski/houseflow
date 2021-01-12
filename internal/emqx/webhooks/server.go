package webhooks

import (
	"context"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"strings"
	"time"

	"github.com/gbaranski/houseflow/pkg/database"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

const uuidLength = 36

// Server struct which holds state of app
type Server struct {
	mongo database.Mongo
}

// NewServer creates new server
func NewServer(db database.Mongo) Server {
	return Server{
		mongo: db,
	}
}

// WebhookEvent when webhook triggered
type WebhookEvent struct {
	Action   string `json:"action"`
	Username string `json:"username"`
	ClientID string `json:"clientid"`
}

// ConnectedEvent when client connects
type ConnectedEvent struct {
	WebhookEvent
	ProtoVersion       int    `json:"proto_ver"`
	KeepAlive          int    `json:"keepalive"`
	IPAddress          string `json:"ipaddress"`
	ConnectedTimestamp uint64 `json:"connected_at"`
}

// DisconnectedEvent when client disconnected
type DisconnectedEvent struct {
	WebhookEvent
	Reason string `json:"reason"`
}

// OnEvent handles /event request
func (s *Server) OnEvent(w http.ResponseWriter, r *http.Request) {
	b, err := ioutil.ReadAll(r.Body)
	defer r.Body.Close()
	fmt.Println("bytes: ", b)
	fmt.Println("str: ", string(b))

	if err != nil {
		http.Error(w, err.Error(), http.StatusUnprocessableEntity)
		log.Println("Failed reading request body: ", err.Error())
		return
	}
	var e WebhookEvent
	if err := json.Unmarshal(b, &e); err != nil {
		http.Error(w, err.Error(), http.StatusUnprocessableEntity)
		log.Println("Failed unmarhsalling body: ", err.Error())
		return
	}

	if !strings.HasPrefix(e.ClientID, "device_") {
		return
	}

	if e.Action != "client_connected" && e.Action != "client_disconnected" {
		msg := "Invalid action name"
		fmt.Fprintf(w, msg)
		log.Println(msg)
		return
	}

	deviceID, err := primitive.ObjectIDFromHex(e.Username)
	if err != nil {
		fmt.Fprintf(w, err.Error())
		log.Printf("Error when parsing ObjectID: %s, err: %s", e.Username, err.Error())
		return
	}
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*5)
	defer cancel()

	if e.Action == "client_connected" {
		err = s.mongo.UpdateDeviceOnlineState(ctx, deviceID, true)
	} else if e.Action == "client_disconnected" {
		err = s.mongo.UpdateDeviceOnlineState(ctx, deviceID, false)
	}
	if err != nil {
		fmt.Fprintf(w, err.Error())
		log.Println(err)
	}
	fmt.Fprintf(w, "Success!")
}
