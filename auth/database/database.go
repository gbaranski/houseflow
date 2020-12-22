package database

import (
	"context"
	"errors"
	"fmt"
	"log"
	"time"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"go.mongodb.org/mongo-driver/mongo/readpref"
)

// Database struct
type Database struct {
	db              *mongo.Database
	Client          *mongo.Client
	usersCollection *mongo.Collection
}

// CreateDatabase creates and connect to DB
//
// Returns Database struct
func CreateDatabase(ctx context.Context, username string, password string) (*Database, error) {
	client, err := mongo.Connect(ctx, options.Client().ApplyURI(fmt.Sprintf("mongodb://%s:%s@mongo:27017", username, password)))
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
		log.Panicln("Failed creating index", err)
	}
	log.Println("Created unique index for usersCollection name:", name)

	return &Database{
		db:              db,
		Client:          client,
		usersCollection: db.Collection("users"),
	}, nil
}

// AddUser adds user to db
func (db *Database) AddUser(user *User) error {
	err := user.hashPassword()
	if err != nil {
		return err
	}

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	res, err := db.usersCollection.InsertOne(ctx, user)
	if err != nil {
		return err
	}
	log.Println("Inserted user to database with ID: ", res.InsertedID)
	return nil
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
