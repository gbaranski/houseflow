package main

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	paho "github.com/eclipse/paho.mqtt.golang"
	"github.com/gbaranski/houseflow/pkg/database"
	"github.com/gbaranski/houseflow/pkg/mqtt"
	"github.com/gbaranski/houseflow/pkg/utils"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

var (
	mongoUsername = utils.MustGetEnv("MONGO_INITDB_ROOT_USERNAME")
	mongoPassword = utils.MustGetEnv("MONGO_INITDB_ROOT_PASSWORD")
	databaseName  = utils.MustGetEnv("DB_NAME")
	serviceName   = utils.MustGetEnv("SERVICE_NAME")
	serviceID     = utils.MustGetEnv("SERVICE_ID")
)

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
		ClientID:    fmt.Sprintf("%s-%s", serviceName, serviceID),
		BrokerURL:   "tcp://emqx:1883/mqtt",
		KeepAlive:   time.Second * 30,
		PingTimeout: time.Second * 5,
	})
	if err != nil {
		panic(err)
	}

	if token := mqtt.Client.Subscribe("+/reportState", 0, func(c paho.Client, m paho.Message) {
    deviceID, err := primitive.ObjectIDFromHex(m.Topic()[:24])
    if err != nil {
      fmt.Println("fail update state, deviceID invalid: ", err.Error())
      return
    }
    var state map[string]interface{}
    err = json.Unmarshal(m.Payload(), &state)
    if err != nil {
      fmt.Println("fail marshal state: ", err.Error())
      return
    }

  	ctx, cancel := context.WithTimeout(context.Background(), time.Second*5)
    defer cancel()
    err = mongo.UpdateDeviceState(ctx, deviceID, state)
    if err != nil {
      fmt.Println("fail update device state: ", err.Error())
      return
    }
    fmt.Printf("Success updating state of Device ID: %s", deviceID.Hex())
	}); token.Wait() && token.Error() != nil {
		panic(token.Error())
	}

	select {}
}
