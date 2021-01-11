package main

import (
	"context"
	"time"

	"github.com/gbaranski/houseflow/internal/auth"
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

	mongo, err := database.NewMongo(ctx, database.MongoOptions{
		Username:     mongoUsername,
		Password:     mongoPassword,
		DatabaseName: "houseflowDB",
	})

	redis, err := database.NewRedis(ctx, database.RedisOptions{})
  if err != nil {
    panic(err)
  }

	s := auth.NewServer(mongo, redis)
	err = s.Router.Run(":80")
	if err != nil {
		panic(err)
	}
}
