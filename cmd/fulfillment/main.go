package main

import (
	"context"
	"crypto/ed25519"
	"fmt"
	"log"
	"net/http"
	"time"

	"github.com/caarlos0/env/v6"

	"github.com/gbaranski/houseflow/internal/fulfillment"
	"github.com/gbaranski/houseflow/pkg/database/postgres"
	"github.com/gbaranski/houseflow/pkg/devmgmt"
	"github.com/gbaranski/houseflow/pkg/utils"
)

func main() {
	log.Println("Starting fulfillment service")
	var (
		postgresOptions    postgres.Options
		fulfillmentOptions fulfillment.Options
		devmgmtOptions     = devmgmt.Options{
			ClientID:         utils.MustGetEnv("SERVICE_NAME"),
			ServerPublicKey:  utils.MustParseEnvKey("SERVER_PUBLIC_KEY", ed25519.PrivateKeySize),
			ServerPrivateKey: utils.MustParseEnvKey("SERVER_PRIVATE_KEY", ed25519.PrivateKeySize),
		}
	)
	if err := env.Parse(&postgresOptions); err != nil {
		panic(fmt.Errorf("fail load postgres opts %s", err.Error()))
	}
	if err := env.Parse(&fulfillmentOptions); err != nil {
		panic(fmt.Errorf("fail load fulfillment opts %s", err.Error()))
	}
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	postgres, err := postgres.New(ctx, postgresOptions)
	if err != nil {
		panic(err)
	}
	devmgmt, err := devmgmt.New(devmgmtOptions)
	if err != nil {
		panic(err)
	}

	fulfillment := fulfillment.New(postgres, devmgmt, fulfillmentOptions)

	http.ListenAndServe(":80", fulfillment.Router)
	if err != nil {
		panic(err)
	}
}
