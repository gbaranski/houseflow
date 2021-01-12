package database

import (
	"context"

	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/gbaranski/houseflow/pkg/utils"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

// GetUserByEmail returns found user from DB, query by email
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

// GetUserByID returns found user from DB, query by user ID
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
