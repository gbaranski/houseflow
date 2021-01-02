package main

import (
	"context"
	"log"
	"net/http"
	"time"

	"github.com/gbaranski/houseflow/webhooks/database"
	server "github.com/gbaranski/houseflow/webhooks/server"
)

func main() {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	log.Println("Starting webhooks")
	db, err := database.CreateDatabase(ctx)
	if err != nil {
		panic(err)
	}
	s := server.NewServer(db)
	http.HandleFunc("/event", func(w http.ResponseWriter, req *http.Request) {
		s.OnEvent(w, req)
	})

	http.ListenAndServe(":80", nil)
}
