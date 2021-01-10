package database

import (
	"context"
	"fmt"
	"log"

	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/gbaranski/houseflow/pkg/utils"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
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
  Users *mongo.Collection
  Devices *mongo.Collection

}

//  Mongo contains db and client
type Mongo struct {
	db              *mongo.Database
	Client          *mongo.Client
  Collections     Collections
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
    Users: users,
    Devices: devices,
  },nil
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
		db:              db,
		Client:          client,
    Collections: collections,
	}, nil

}

// GetUserbyEmail returns found user from DB, query by email
func (m *Mongo) GetUserByEmail(ctx context.Context, email string) (types.User, error) {
	result := m.Collections.Users.FindOne(ctx, bson.M{"email": email})
	if result.Err() != nil {
		return types.User{}, result.Err()
	}

	var user types.User
	if err := result.Decode(&user); err != nil {
		return types.User{}, err
	}

	return user, nil
}

// GetUserbyEmail returns found user from DB, query by email
func (m *Mongo) GetUserByID(ctx context.Context, id primitive.ObjectID) (types.User, error) {
	result := m.Collections.Users.FindOne(ctx, bson.M{"_id": id})
	if result.Err() != nil {
		return types.User{}, result.Err()
	}

	var user types.User
	if err := result.Decode(&user); err != nil {
		return types.User{}, err
	}

	return user, nil
}

// AddUser adds user to db
func (m *Mongo) AddUser(ctx context.Context, user types.User) (primitive.ObjectID, error) {
	password, err := utils.HashPassword([]byte(user.Password))
	if err != nil {
		return primitive.ObjectID{}, err
	}

	user.Password = string(password)
	user.Devices = []string{}

	res, err := m.Collections.Users.InsertOne(ctx, user)
	if err != nil {
		return primitive.ObjectID{}, err
	}
	id := res.InsertedID.(primitive.ObjectID)
	return id, nil
}

// createIndexForUnique creates unique index for speicfic key
func createIndexForUnique(ctx context.Context, collection *mongo.Collection, key string) (string, error) {
	model := mongo.IndexModel{
		Keys:    bson.D{{Key: key, Value: 1}},
		Options: options.Index().SetUnique(true),
	}
	return collection.Indexes().CreateOne(ctx, model)
}

// UpdateDeviceState updates "state" property on device
func (m *Mongo) UpdateDeviceState(ctx context.Context, deviceID primitive.ObjectID, state map[string]interface{}) error {
	result, err := m.Collections.Users.UpdateOne(ctx,
		bson.M{"_id": deviceID},
		bson.D{{"$set", bson.D{{"state", state}}}})
	if err != nil {
		return err
	}
	if result.ModifiedCount < 1 {
		return fmt.Errorf("no document modified")
	}
  return nil
}

// UpdateDeviceOnlineState modifies only the state.online property to "online" arg
func (m *Mongo) UpdateDeviceOnlineState(ctx context.Context, deviceID primitive.ObjectID, online bool) error {
	res, err := m.Collections.Devices.UpdateOne(ctx, bson.M{"_id": deviceID}, bson.M{
		"$set": bson.M{
			"state.online": online}})
	if err != nil {
		return err
	}
	if res.ModifiedCount < 1 {
		return fmt.Errorf("Not matched any devices")
	}

	return nil
}

// GetUserDevices retreives user devices
func (m *Mongo) GetDevicesByIDs(ctx context.Context, deviceIDs []primitive.ObjectID) ([]types.Device, error) {
	cur, err := m.Collections.Devices.Find(ctx, bson.M{"_id": bson.M{"$in": deviceIDs}})
	if err != nil {
		return nil, err
	}
	var devices []types.Device
	if err = cur.All(ctx, &devices); err != nil {
		return nil, err
	}
	return devices, nil
}

