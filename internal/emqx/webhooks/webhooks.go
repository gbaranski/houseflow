package webhooks

import (
	"context"
	"crypto/ed25519"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"time"

	"github.com/go-chi/chi"
	"github.com/go-chi/chi/middleware"
)

// Database for webhooks
type Database interface {
	UpdateDeviceOnlineState(ctx context.Context, ID string, state bool) error
}

// Options webhook options
type Options struct {
	// Public key of server
	//
	// *Required*
	ServerPublicKey ed25519.PublicKey
}

// Webhooks struct which holds state of app
type Webhooks struct {
	db     Database
	opts   Options
	Router *chi.Mux
}

// New creates new webhooks
func New(db Database, opts Options) Webhooks {
	wh := Webhooks{
		db:     db,
		Router: chi.NewRouter(),
		opts:   opts,
	}
	wh.Router.Use(middleware.Logger)
	wh.Router.Use(middleware.Recoverer)
	wh.Router.Use(middleware.RealIP)
	wh.Router.Use(middleware.Heartbeat("/ping"))
	wh.Router.Use(middleware.Timeout(time.Second * 10))
	wh.Router.Post("/event", wh.OnEvent)

	return wh
}

// OnEvent handles /event request
func (wh *Webhooks) OnEvent(w http.ResponseWriter, r *http.Request) {
	var e WebhookEvent
	if err := json.NewDecoder(r.Body).Decode(&e); err != nil {
		http.Error(w, err.Error(), http.StatusUnprocessableEntity)
		log.Println("Failed unmarhsalling body: ", err.Error())
		return
	}

	pkeyenc, err := base64.StdEncoding.DecodeString(e.Username)
	if err != nil {
		http.Error(w, err.Error(), http.StatusUnauthorized)
		return
	}
	if len(pkeyenc) != ed25519.PublicKeySize {
		http.Error(w, err.Error(), http.StatusUnauthorized)
		return
	}
	// Skipping webhooks cause we don't care about some service connection
	if wh.opts.ServerPublicKey.Equal(ed25519.PublicKey(pkeyenc)) {
		return
	}

	if e.Action != "client_connected" && e.Action != "client_disconnected" {
		http.Error(w, "invalid action name", http.StatusBadRequest)
		return
	}

	ctx, cancel := context.WithTimeout(context.Background(), time.Second*5)
	defer cancel()

	if e.Action == "client_connected" {
		err = wh.db.UpdateDeviceOnlineState(ctx, e.Username, true)
	} else if e.Action == "client_disconnected" {
		err = wh.db.UpdateDeviceOnlineState(ctx, e.Username, false)
	}
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	fmt.Fprintf(w, "Success!")
}
