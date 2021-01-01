package database

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"os"
	"time"

	mqtt "github.com/eclipse/paho.mqtt.golang"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"go.mongodb.org/mongo-driver/mongo/readpref"
)

const mongoUsernameEnv string = "MONGO_INITDB_ROOT_USERNAME"
const mongoPasswordEnv string = "MONGO_INITDB_ROOT_PASSWORD"

// Mongo contains db and client
type Mongo struct {
	db                *mongo.Database
	Client            *mongo.Client
	devicesCollection *mongo.Collection
}

func getMongoCredentials() (*string, *string, error) {
	username, present := os.LookupEnv(mongoUsernameEnv)
	if !present {
		return nil, nil, fmt.Errorf("%s not set in .env", mongoUsernameEnv)
	}
	password, present := os.LookupEnv(mongoPasswordEnv)
	if !present {
		return nil, nil, fmt.Errorf("%s not set in .env", mongoPasswordEnv)
	}
	return &username, &password, nil
}

func createMongo(ctx context.Context) (*Mongo, error) {
	username, password, err := getMongoCredentials()
	if err != nil {
		return nil, err
	}
	client, err := mongo.Connect(ctx, options.Client().ApplyURI(fmt.Sprintf("mongodb://%s:%s@mongo:27017", *username, *password)))
	if err != nil {
		return nil, err
	}
	db := client.Database("houseflowDB")

	// Ping the primary
	if err := client.Ping(ctx, readpref.Primary()); err != nil {
		return nil, err
	}
	log.Println("Successfully connected and pinged.")

	return &Mongo{
		db:                db,
		Client:            client,
		devicesCollection: db.Collection("devices"),
	}, nil

}

func (m *Mongo) UpdateDeviceState(c mqtt.Client, msg mqtt.Message) {
	fmt.Println("Updating device state")
	deviceID := msg.Topic()[:24]
	objID, err := primitive.ObjectIDFromHex(deviceID)
	if err != nil {
		fmt.Printf("Failed converting %s to ObjectID", deviceID)
		return
	}

	var parsedJSON interface{}
	err = json.Unmarshal(msg.Payload(), &parsedJSON)
	if err != nil {
		fmt.Println("Error occured when parsing JSON from ", deviceID)
	}

	ctx, cancel := context.WithTimeout(context.Background(), time.Second*5)
	defer cancel()
	result, err := m.devicesCollection.UpdateOne(ctx,
		bson.M{"_id": objID},
		bson.D{{"$set", bson.D{{"state", parsedJSON}}}})
	if err != nil {
		fmt.Println("Error when updating device state", err.Error())
		return
	}
	if result.ModifiedCount < 1 {
		fmt.Println("Not modified any document")
		return
	}

	fmt.Printf("Successfully changed state of %s", deviceID)
}
