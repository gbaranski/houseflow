package main

import (
	"context"
	"log"
	"os"
	"time"

	mqtt "github.com/eclipse/paho.mqtt.golang"
	"github.com/gbaranski/houseflow/actions/config"
	"github.com/gbaranski/houseflow/actions/database"
	"github.com/gbaranski/houseflow/actions/server"
)

func initMQTT() mqtt.Client {
	mqtt.ERROR = log.New(os.Stdout, "[ERROR] ", 0)
	mqtt.CRITICAL = log.New(os.Stdout, "[CRIT] ", 0)
	mqtt.WARN = log.New(os.Stdout, "[WARN]  ", 0)
	// mqtt.DEBUG = log.New(os.Stdout, "[DEBUG] ", 0)

	// Add there some kind of password soon
	opts := mqtt.
		NewClientOptions().
		AddBroker("tcp://emqx:1883/mqtt").
		SetClientID("service-actions-1").
		SetKeepAlive(5 * time.Second).
		SetPingTimeout(1 * time.Second)

	c := mqtt.NewClient(opts)
	if token := c.Connect(); token.Wait() && token.Error() != nil {
		panic(token.Error())
	}
	return c
}

func main() {
	config, err := config.Load()
	if err != nil {
		panic(err)
	}

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	db, err := database.CreateDatabase(ctx)
	defer cancel()

	mqttc := initMQTT()
	defer mqttc.Disconnect(240)

	s := server.NewServer(db, mqttc, *config)
	err = s.Router.Run(":80")
	if err != nil {
		panic(err)
	}
}
