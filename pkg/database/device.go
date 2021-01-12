package database

import (
	"context"
	"fmt"

	"github.com/gbaranski/houseflow/pkg/types"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

// GetDevicesByIDs retreives devices by array of IDs
func (m *Mongo) GetDevicesByIDs(ctx context.Context, deviceIDs []primitive.ObjectID) ([]types.Device, error) {
	cur, err := m.Collections.Devices.Find(ctx,
		bson.M{"_id": bson.M{
			"$in": deviceIDs,
		},
		})
	if err != nil {
		return nil, err
	}
	var devices []types.Device
	if err = cur.All(ctx, &devices); err != nil {
		return nil, err
	}
	return devices, nil
}

// UpdateDeviceState updates "state" property on device
func (m *Mongo) UpdateDeviceState(ctx context.Context, deviceID primitive.ObjectID, state map[string]interface{}) error {
	result, err := m.Collections.Users.UpdateOne(ctx,
		bson.M{"_id": deviceID},
		bson.M{
			"$set": bson.M{
				"state": state,
			},
		})
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

// AddDevice adds device to mongoDB
func (m *Mongo) AddDevice(ctx context.Context, device types.Device) (primitive.ObjectID, error) {
	res, err := m.Collections.Devices.InsertOne(ctx, device)
	if err != nil {
		return primitive.ObjectID{}, err
	}
	id := res.InsertedID.(primitive.ObjectID)
	return id, nil
}
