package main

import (
	"context"
	"time"

	"github.com/gbaranski/houseflow/internal/emqx/auth"
	"github.com/gbaranski/houseflow/pkg/database"
	"github.com/gbaranski/houseflow/pkg/utils"
)

var (
	mongoUsername = utils.MustGetEnv("MONGO_INITDB_ROOT_USERNAME")
	mongoPassword = utils.MustGetEnv("MONGO_INITDB_ROOT_PASSWORD")
	privateKey    = utils.MustGetEnv("SERVER_PRIVATE_KEY")
	serviceName   = utils.MustGetEnv("SERVICE_NAME")
	serviceID     = utils.MustGetEnv("SERVICE_ID")
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
	s := auth.NewServer(mongo)
	s.Router.Run(":80")
}
