package main

import (
	"context"
	"fmt"
	"time"

	"github.com/gbaranski/houseflow/actions/server"
	"github.com/gbaranski/houseflow/pkg/database"
	"github.com/gbaranski/houseflow/pkg/mqtt"
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
		Enabled:      true,
		Username:     mongoUsername,
		Password:     mongoPassword,
		DatabaseName: "houseflowDB",
	})

	mqttc := mqtt.NewMQTT(mqtt.MQTTOptions{
		ClientID:    fmt.Sprintf("%s-%s", serviceName, serviceID),
		BrokerURL:   "tcp://emqx:1883/mqtt",
		KeepAlive:   time.Second * 30,
		PingTimeout: time.Second * 5,
		PrivateKey:  []byte(privateKey),
	})

	s := server.NewServer(mongo, mqttc)
	err = s.Router.Run(":80")
	if err != nil {
		panic(err)
	}
}
