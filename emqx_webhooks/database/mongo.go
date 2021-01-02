package database

import (
	"context"
	"fmt"
	"log"
	"os"
	"time"

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

type DeviceInfo struct {
	Manufacturer string `bson:"manufacturer"`
	Model        string `bson:"model"`
	HwVersion    string `bson:"hwVersion"`
	SwVersion    string `bson:"swVersion"`
}

type DeviceName struct {
	DefaultNames []string `bson:"defaultNames"`
	Name         string   `bson:"name"`
	Nicknames    []string `bson:"nicknames"`
}

type Device struct {
	ID              primitive.ObjectID `bson:"_id"`
	Type            string             `bson:"type"`
	Traits          []string           `bson:"traits"`
	Name            DeviceName         `bson:"name"`
	DeviceInfo      DeviceInfo         `bson:"deviceInfo"`
	WillReportState bool               `bson:"willReportState"`
	RoomHint        string             `bson:"roomHint"`
	State           interface{}        `bson:"state"`
}

func (m *Mongo) UpdateDeviceOnlineState(deviceID primitive.ObjectID, online bool) error {
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	res, err := m.devicesCollection.UpdateOne(ctx, bson.M{"_id": deviceID}, bson.M{
		"$set": bson.M{
			"state.online": online}})
	if err != nil {
		return err
	}
	if res.ModifiedCount < 1 {
		return fmt.Errorf("Not matched any devices")
	}

	log.Printf("Successfully updated online state to %t ID: %s", online, deviceID.Hex())
	return nil
}
