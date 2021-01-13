package main

import (
	"context"
	"crypto/ed25519"
	"encoding/base64"
	"time"

	"github.com/gbaranski/houseflow/internal/fulfillment"
	"github.com/gbaranski/houseflow/pkg/database"
	"github.com/gbaranski/houseflow/pkg/mqtt"
	"github.com/gbaranski/houseflow/pkg/utils"
)

var (
	mongoUsername = utils.MustGetEnv("MONGO_INITDB_ROOT_USERNAME")
	mongoPassword = utils.MustGetEnv("MONGO_INITDB_ROOT_PASSWORD")
	accessKey     = utils.MustGetEnv("ACCESS_KEY")
	privateKey    ed25519.PrivateKey
)

func init() {
	skey, err := base64.StdEncoding.DecodeString(utils.MustGetEnv("SERVER_PRIVATE_KEY"))
	if err != nil {
		panic(err)
	}
	privateKey = ed25519.PrivateKey(skey)
}

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

	mqtt, err := mqtt.NewMQTT(mqtt.Options{
		ClientID:    "fulfillment",
		BrokerURL:   "tcp://emqx:1883/mqtt",
		KeepAlive:   time.Second * 30,
		PingTimeout: time.Second * 5,
		PrivateKey:  []byte(privateKey),
	})
	if err != nil {
		panic(err)
	}

	f := fulfillment.NewFulfillment(mongo, mqtt, fulfillment.Options{
		AccessKey: accessKey,
	})
	err = f.Router.Run(":80")
	if err != nil {
		panic(err)
	}
}
