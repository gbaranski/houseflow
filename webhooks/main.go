package main

import (
	"context"
	"log"
	"net/http"
	"os"

	server "github.com/gbaranski/homeflow/webhooks/server"
	services "github.com/gbaranski/homeflow/webhooks/services"
)

func main() {
	PORT := os.Getenv("PORT_WEBHOOKS")
	if len(PORT) == 0 {
		log.Fatalln("Port is not defined")
	}
	firebaseClient, error := services.InitFirebase(context.Background())
	if error != nil {
		log.Fatalln(error)
	}
	log.Println("Starting webhooks")
	http.HandleFunc("/event", func(w http.ResponseWriter, req *http.Request) {
		server.OnEvent(w, req, firebaseClient)
	})

	http.ListenAndServe(":"+PORT, nil)
}
