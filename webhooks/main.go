package main

import (
	"context"
	"log"
	"net/http"

	server "github.com/gbaranski/homeflow/webhooks/server"
	services "github.com/gbaranski/homeflow/webhooks/services"
)

func main() {
	firebaseClient, error := services.InitFirebase(context.Background())
	if error != nil {
		log.Fatalln(error)
	}
	PORT := "8001"
	log.Println("Starting webhooks")
	http.HandleFunc("/event", func(w http.ResponseWriter, req *http.Request) {
		server.OnEvent(w, req, firebaseClient)
	})

	http.ListenAndServe(":"+PORT, nil)
}
