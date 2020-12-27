package main

import (
	"context"
	"log"
	"os"
	"time"

	database "github.com/gbaranski/houseflow/auth/database"
	server "github.com/gbaranski/houseflow/auth/server"
	utils "github.com/gbaranski/houseflow/auth/utils"
)

func main() {
	_, present := os.LookupEnv(utils.JWTAccessKey)
	if !present {
		log.Panicf("%s not set in .env\n", utils.JWTAccessKey)
	}
	_, present = os.LookupEnv(utils.JWTRefreshKey)
	if !present {
		log.Panicf("%s not set in .env\n", utils.JWTRefreshKey)
	}

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	db, err := database.CreateDatabase(ctx)
	if err != nil {
		log.Panicln("Error when creating database", err)
	}

	defer func() {
		ctx, cancel := context.WithTimeout(context.Background(), time.Second*5)
		defer cancel()
		if err = db.Mongo.Client.Disconnect(ctx); err != nil {
			log.Panicln("Error when MongoClient.Disconnect()", err.Error())
		}
	}()
	server := server.NewServer(db)
	err = server.Router.Run(":80")
	if err != nil {
		log.Panicln("Error when starting server", err)
	}
}
