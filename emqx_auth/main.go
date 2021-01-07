package main

import (
	"context"
	"time"

	"github.com/gbaranski/houseflow/emqx_auth/database"
	"github.com/gbaranski/houseflow/emqx_auth/server"
)

func main() {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	db, err := database.CreateDatabase(ctx)
	if err != nil {
		panic(err)
	}
	s := server.NewServer(db)
	s.Router.Run(":80")
}
