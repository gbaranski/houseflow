package main

import (
	"context"
	"time"

	"github.com/gbaranski/houseflow/actions/database"
	"github.com/gbaranski/houseflow/actions/server"
)

type test struct {
	Something string `binding:"eq=action.devices.SYNC"`
}

func main() {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	db, err := database.CreateDatabase(ctx)
	defer cancel()
	s := server.NewServer(db)
	err = s.Router.Run(":80")
	if err != nil {
		panic(err)
	}
}
