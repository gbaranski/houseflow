package main

import (
	"context"
	"log"
	"net/http"
	"time"

	"github.com/gbaranski/houseflow/internal/emqx/webhooks"
	"github.com/gbaranski/houseflow/pkg/database"
	"github.com/gbaranski/houseflow/pkg/utils"
)

var (
	mongoUsername = utils.MustGetEnv("MONGO_INITDB_ROOT_USERNAME")
	mongoPassword = utils.MustGetEnv("MONGO_INITDB_ROOT_PASSWORD")
)

func main() {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	log.Println("Starting webhooks")
	mongo, err := database.NewMongo(ctx, database.MongoOptions{
		Username:     mongoUsername,
		Password:     mongoPassword,
		DatabaseName: "houseflowDB",
	})
	if err != nil {
		panic(err)
	}
	s := webhooks.NewServer(mongo)

	http.HandleFunc("/event", func(w http.ResponseWriter, req *http.Request) {
		s.OnEvent(w, req)
	})

	err = http.ListenAndServe(":80", nil)
	if err != nil {
		panic(err)
	}
}
