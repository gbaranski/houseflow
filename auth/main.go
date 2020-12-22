package main

import (
	"context"
	"log"
	"os"
	"time"

	database "github.com/gbaranski/houseflow/auth/database"
	server "github.com/gbaranski/houseflow/auth/server"
)

const mongoUsernameEnv string = "MONGO_INITDB_ROOT_USERNAME"
const mongoPasswordEnv string = "MONGO_INITDB_ROOT_PASSWORD"

func main() {
	username, present := os.LookupEnv(mongoUsernameEnv)
	if !present {
		log.Panicf("%s not set in .env\n", mongoUsernameEnv)
	}
	password, present := os.LookupEnv(mongoPasswordEnv)
	if !present {
		log.Panicf("%s not set in .env\n", mongoPasswordEnv)
	}
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	db, err := database.CreateDatabase(ctx, username, password)
	if err != nil {
		log.Panicln("Error when creating database", err)
	}
	defer func() {
		ctx, cancel := context.WithTimeout(context.Background(), time.Second*5)
		defer cancel()
		if err = db.Client.Disconnect(ctx); err != nil {
			log.Panicln("Error when MongoClient.Disconnect()", err.Error())
		}
	}()
	server := server.CreateServer(db)
	err = server.Start()
	if err != nil {
		log.Panicln("Error when starting server", err)
	}
}
