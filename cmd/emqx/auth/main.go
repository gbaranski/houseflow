package main

import (
	"context"
	"crypto/ed25519"
	"fmt"
	"log"
	"net/http"
	"time"

	"github.com/caarlos0/env/v6"

	"github.com/gbaranski/houseflow/internal/emqx/auth"
	"github.com/gbaranski/houseflow/pkg/database/postgres"
	"github.com/gbaranski/houseflow/pkg/utils"
)

func main() {
	log.Println("Starting emqx/auth service")
	var (
		postgresOptions postgres.Options
		authOptions     = auth.Options{
			ServerPublicKey: utils.MustParseEnvKey("SERVER_PUBLIC_KEY", ed25519.PublicKeySize),
		}
	)
	if err := env.Parse(&postgresOptions); err != nil {
		panic(fmt.Errorf("fail load postgres opts %s", err.Error()))
	}

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	postgres, err := postgres.New(ctx, postgresOptions)
	if err != nil {
		panic(err)
	}

	auth := auth.New(postgres, authOptions)

	http.ListenAndServe(":80", auth.Router)
	if err != nil {
		panic(err)
	}
}
