package database

import (
	"context"
	"fmt"
	"log"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"go.mongodb.org/mongo-driver/mongo/readpref"
)

// MongoOptions defines options for mongoDB, use NewMongoOptions for constructor
type MongoOptions struct {
	// Username for mongoDB
	//
	// Default: "root"
	Username string

	// Password for mongoDB
	//
	// Default: "example"
	Password string

	// Name of the database,
	//
	// Default: houseflowDB
	DatabaseName string
}

// Parse parses the options and set the defaults
func (opts *MongoOptions) Parse() {
	if opts.DatabaseName == "" {
		opts.DatabaseName = "houseflowDB"
	}
	if opts.Username == "" {
		opts.Username = "root"
	}
	if opts.Password == "" {
		opts.Password = "example"
	}
}

// Collections hold avaialble connections for houseflowDB
type Collections struct {
	Users   *mongo.Collection
	Devices *mongo.Collection
}

// Mongo contains db and client
type Mongo struct {
	db          *mongo.Database
	Client      *mongo.Client
	Collections Collections
}

func newCollections(ctx context.Context, db *mongo.Database) (Collections, error) {
	users := db.Collection("users")
	name, err := createIndexForUnique(ctx, users, "email")
	if err != nil {
		return Collections{}, err
	}
	log.Println("Created unique index for usersCollection name:", name)

	devices := db.Collection("devices")
	return Collections{
		Users:   users,
		Devices: devices,
	}, nil
}

// NewMongo creates Mongo struct and returns it, connects to mongoDB with given options
func NewMongo(ctx context.Context, opts MongoOptions) (Mongo, error) {
	opts.Parse()
	client, err := mongo.Connect(ctx, options.Client().ApplyURI(fmt.Sprintf("mongodb://%s:%s@mongo:27017", opts.Username, opts.Password)))
	if err != nil {
		return Mongo{}, err
	}
	// Ping the primary
	if err := client.Ping(ctx, readpref.Primary()); err != nil {
		return Mongo{}, err
	}

	log.Println("Successfully connected and pinged.")

	db := client.Database("houseflowDB")
	collections, err := newCollections(ctx, db)
	if err != nil {
		return Mongo{}, err
	}

	return Mongo{
		db:          db,
		Client:      client,
		Collections: collections,
	}, nil

}

// createIndexForUnique creates unique index for speicfic key
func createIndexForUnique(ctx context.Context, collection *mongo.Collection, key string) (string, error) {
	model := mongo.IndexModel{
		Keys:    bson.D{{Key: key, Value: 1}},
		Options: options.Index().SetUnique(true),
	}
	return collection.Indexes().CreateOne(ctx, model)
}
