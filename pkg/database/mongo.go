package database

import (
	"context"
	"errors"
	"fmt"
	"log"
	"os"
	"time"

	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/gbaranski/houseflow/pkg/utils"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"go.mongodb.org/mongo-driver/mongo/readpref"
)

const mongoUsernameEnv string = "MONGO_INITDB_ROOT_USERNAME"
const mongoPasswordEnv string = "MONGO_INITDB_ROOT_PASSWORD"

// MongoOptions defines options for mongoDB
type MongoOptions struct {
	// Defines if should connect to Mongo
	//
	// Default: false
	Enabled bool
}

// Mongo contains db and client
type Mongo struct {
	db              *mongo.Database
	Client          *mongo.Client
	usersCollection *mongo.Collection
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

	usersCollection := db.Collection("users")
	name, err := createIndexForUnique(usersCollection, "email")
	if err != nil {
		return nil, err
	}
	log.Println("Created unique index for usersCollection name:", name)

	return &Mongo{
		db:              db,
		Client:          client,
		usersCollection: usersCollection,
	}, nil

}

// GetUser returns found user from DB
func (m *Mongo) GetUser(email string) (*types.User, error) {
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	result := m.usersCollection.FindOne(ctx, bson.M{"email": email})
	if result.Err() != nil {
		return nil, result.Err()
	}

	var user *types.User
	if err := result.Decode(&user); err != nil {
		return nil, err
	}

	return user, nil
}

// AddUser adds user to db
func (m *Mongo) AddUser(user types.User) (*primitive.ObjectID, error) {
	password, err := utils.HashPassword(user.Password)
	if err != nil {
		return nil, err
	}

	user.Password = string(password)
	user.Devices = []string{}

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	res, err := m.usersCollection.InsertOne(ctx, user)
	if err != nil {
		return nil, err
	}
	id := res.InsertedID.(primitive.ObjectID)
	return &id, nil
}

func createIndexForUnique(collection *mongo.Collection, key string) (string, error) {
	model := mongo.IndexModel{
		Keys:    bson.D{{Key: key, Value: 1}},
		Options: options.Index().SetUnique(true),
	}
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)

	defer cancel()

	return collection.Indexes().CreateOne(ctx, model)
}

// IsDuplicateError checks if mongo write error is about duplicate
func IsDuplicateError(err error) bool {
	var e mongo.WriteException
	if errors.As(err, &e) {
		for _, we := range e.WriteErrors {
			if we.Code == 11000 {
				return true
			}
		}
	}
	return false
}
