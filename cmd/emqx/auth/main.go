package main

import (
	"context"
	"crypto/ed25519"
	"net/http"
	"time"

	"github.com/gbaranski/houseflow/internal/emqx/auth"
	"github.com/gbaranski/houseflow/pkg/database"
	"github.com/gbaranski/houseflow/pkg/utils"
)

var (
	mongoUsername = utils.MustGetEnv("MONGO_INITDB_ROOT_USERNAME")
	mongoPassword = utils.MustGetEnv("MONGO_INITDB_ROOT_PASSWORD")

	serverPublicKey = ed25519.PublicKey(utils.MustGetEnv("SERVER_PUBLIC_KEY"))
)

func main() {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	mongo, err := database.NewMongo(ctx, database.MongoOptions{
		Username:     mongoUsername,
		Password:     mongoPassword,
		DatabaseName: "houseflowDB",
	})
	if err != nil {
		panic(err)
	}
	s := auth.New(mongo, auth.Options{
		ServerPublicKey: serverPublicKey,
	})
	err = http.ListenAndServe(":80", s.Router)
	if err != nil {
		panic(err)
	}
}
