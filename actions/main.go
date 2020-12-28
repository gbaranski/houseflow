package main

import (
	"context"
	"log"
	"time"

	database "github.com/gbaranski/houseflow/actions/database"
	server "github.com/gbaranski/houseflow/actions/server"
)

func main() {
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
