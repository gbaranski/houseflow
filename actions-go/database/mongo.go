package database

import (
	"context"
	"errors"
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

	return &Mongo{
		db:              db,
		Client:          client,
		usersCollection: db.Collection("users"),
	}, nil

}

// GetUserByEmail returns found user from DB, query by email
func (m *Mongo) GetUserByEmail(email string) (*User, error) {
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	result := m.usersCollection.FindOne(ctx, bson.M{"email": email})
	if result.Err() != nil {
		return nil, result.Err()
	}

	var user *User
	if err := result.Decode(&user); err != nil {
		return nil, err
	}

	return user, nil
}

// GetUserByID returns found user from DB, query by _id
func (m *Mongo) GetUserByID(ID primitive.ObjectID) (*User, error) {
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	result := m.usersCollection.FindOne(ctx, bson.M{"_id": ID})
	if result.Err() != nil {
		return nil, result.Err()
	}

	var user *User
	if err := result.Decode(&user); err != nil {
		return nil, err
	}

	return user, nil
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
