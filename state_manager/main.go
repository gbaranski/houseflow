package main

import (
	"context"
	"log"
	"os"
	"time"

	mqtt "github.com/eclipse/paho.mqtt.golang"
	"github.com/gbaranski/houseflow/state_manager/database"
)

func main() {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	db, err := database.CreateDatabase(ctx)
	if err != nil {
		panic(err)
	}
	mqtt.ERROR = log.New(os.Stdout, "[ERROR] ", 0)
	mqtt.CRITICAL = log.New(os.Stdout, "[CRIT] ", 0)
	mqtt.WARN = log.New(os.Stdout, "[WARN]  ", 0)
	// mqtt.DEBUG = log.New(os.Stdout, "[DEBUG] ", 0)

	// Add there some kind of password soon
	opts := mqtt.NewClientOptions().AddBroker("tcp://emqx:1883/mqtt").SetClientID("service-state_manager-1")
	opts.SetKeepAlive(5 * time.Second)
	opts.SetPingTimeout(1 * time.Second)

	c := mqtt.NewClient(opts)
	if token := c.Connect(); token.Wait() && token.Error() != nil {
		panic(token.Error())
	}

	if token := c.Subscribe("+/reportState", 0, db.Mongo.UpdateDeviceState); token.Wait() && token.Error() != nil {
		panic(token.Error())
	}

	defer c.Disconnect(240)

	select {}
}
