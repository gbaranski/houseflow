package main

import (
	"context"
	"log"
	"net/http"

	server "github.com/gbaranski/houseflow/webhooks/server"
	services "github.com/gbaranski/houseflow/webhooks/services"
)

func main() {
	firebaseClient, error := services.InitFirebase(context.Background())
	if error != nil {
		log.Fatalln(error)
	}
	log.Println("Starting webhooks")
	http.HandleFunc("/event", func(w http.ResponseWriter, req *http.Request) {
		server.OnEvent(w, req, firebaseClient)
	})

	http.ListenAndServe(":80", nil)
}
