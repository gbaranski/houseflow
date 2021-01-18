package database

import (
	"context"
	"fmt"

	"github.com/gbaranski/houseflow/pkg/types"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

// GetDeviceByID retreives single device by single ID
func (m Mongo) GetDeviceByID(ctx context.Context, deviceID string) (types.Device, error) {
	deviceObjectID, err := primitive.ObjectIDFromHex(deviceID)
	if err != nil {
		return types.Device{}, err
	}
	res := m.Collections.Devices.FindOne(ctx,
		bson.M{"_id": deviceObjectID})
	if res.Err() != nil {
		return types.Device{}, res.Err()
	}
	var device types.Device
	err = res.Decode(&device)
	if err != nil {
		return types.Device{}, err
	}

	return device, nil
}

// GetDevicesByIDs retreives devices by array of IDs
func (m Mongo) GetDevicesByIDs(ctx context.Context, deviceIDs []string) ([]types.Device, error) {
	var deviceObjectIDs []primitive.ObjectID
	for _, id := range deviceIDs {
		objectID, err := primitive.ObjectIDFromHex(id)
		if err != nil {
			return nil, fmt.Errorf("fail ObjectIDFromHex ID: %s", id)
		}
		deviceObjectIDs = append(deviceObjectIDs, objectID)
	}

	cur, err := m.Collections.Devices.Find(ctx,
		bson.M{"_id": bson.M{
			"$in": deviceObjectIDs,
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
func (m Mongo) UpdateDeviceState(ctx context.Context, deviceID string, state map[string]interface{}) error {
	deviceObjectID, err := primitive.ObjectIDFromHex(deviceID)
	if err != nil {
		return err
	}

	result, err := m.Collections.Devices.UpdateOne(ctx,
		bson.M{"_id": deviceObjectID},
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
func (m Mongo) UpdateDeviceOnlineState(ctx context.Context, deviceID string, online bool) error {
	deviceObjectID, err := primitive.ObjectIDFromHex(deviceID)
	if err != nil {
		return err
	}
	res, err := m.Collections.Devices.UpdateOne(ctx, bson.M{"_id": deviceObjectID}, bson.M{
		"$set": bson.M{
			"state.online": online}})
	if err != nil {
		return err
	}
	if res.ModifiedCount < 1 {
		return fmt.Errorf("not matched any devices")
	}

	return nil
}

// AddDevice adds device to mongoDB
func (m Mongo) AddDevice(ctx context.Context, device types.Device) (id string, err error) {
	res, err := m.Collections.Devices.InsertOne(ctx, device)
	if err != nil {
		return "", err
	}
	id = res.InsertedID.(primitive.ObjectID).Hex()
	return id, nil
}
