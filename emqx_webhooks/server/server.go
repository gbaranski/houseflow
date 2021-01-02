package event

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"strings"

	database "github.com/gbaranski/houseflow/webhooks/database"
	types "github.com/gbaranski/houseflow/webhooks/types"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

const uuidLength = 36

// Server struct which holds state of app
type Server struct {
	db *database.Database
}

func NewServer(db *database.Database) Server {
	return Server{
		db: db,
	}
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
	var e types.WebhookEvent
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
	if e.Action == "client_connected" {
		err = s.db.Mongo.UpdateDeviceOnlineState(deviceID, true)
	} else if e.Action == "client_disconnected" {
		err = s.db.Mongo.UpdateDeviceOnlineState(deviceID, false)
	}
	if err != nil {
		fmt.Fprintf(w, err.Error())
		log.Println(err)
	}
	fmt.Fprintf(w, "Success!")
}
