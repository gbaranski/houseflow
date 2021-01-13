package fulfillment

import (
	"context"
	"fmt"
	"os"
	"testing"

	"github.com/gbaranski/houseflow/pkg/fulfillment"
	"github.com/gbaranski/houseflow/pkg/types"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
)

const (
	// bcrypt hashed "helloworld"
	helloworld = "$2y$12$sVtI/bYDQ3LWKcGlryQYzeo3IFjIYsl4f4bY6isfBaE3MnaPIcc2e"
)

var opts = Options{
	AccessKey: "someAccessKey",
}

var f Fulfillment
var realUser = types.User{
	ID:        primitive.NewObjectID(),
	FirstName: "John",
	LastName:  "Smith",
	Email:     "john.smith@gmail.com",
	Password:  helloworld,
	Devices:   []string{},
}

var devices = []types.Device{
	{
		Device: fulfillment.Device{
			ID:   "5fef44d38948c2002ae590ab",
			Type: "action.devices.types.LIGHT",
			Traits: []string{
				"action.devices.traits.OnOff",
			},
			Name: fulfillment.DeviceName{
				Name: "Night lamp",
				DefaultNames: []string{
					"Night lamp",
				},
				Nicknames: []string{
					"Night lamp",
				},
			},
			WillReportState: true,
			RoomHint:        "Bedroom",
			DeviceInfo: &fulfillment.DeviceInfo{
				Manufacturer: "gbaranski's garage",
				Model:        "Nightlamp",
				HwVersion:    "1.0.0",
				SwVersion:    "0.1.1",
			},
		},
		PublicKey: "jPcGACNcypUyO9T+lR3Y49s9JpxEuKS0/PMtC7g8AuU=",
		State: map[string]interface{}{
			"online": true,
			"on":     false,
		},
	},
}

type TestDatabase struct{}

func (db TestDatabase) AddDevice(ctx context.Context, device types.Device) (primitive.ObjectID, error) {
	return primitive.NewObjectID(), nil
}

func (db TestDatabase) GetUserByID(ctx context.Context, id primitive.ObjectID) (types.User, error) {
	if id == realUser.ID {
		return realUser, nil
	}
	return types.User{}, mongo.ErrNoDocuments
}
func (db TestDatabase) GetDevicesByIDs(ctx context.Context, deviceIDs []primitive.ObjectID) ([]types.Device, error) {
	var found []types.Device
	for _, e := range deviceIDs {
		for _, v := range devices {
			if e == v.ID {
				found = append(found, v)
			}
		}
	}
	return found, nil
}

func (db TestDatabase) UpdateDeviceState(ctx context.Context, deviceID primitive.ObjectID, state map[string]interface{}) error {
	for _, e := range devices {
		if e.ID == deviceID {
			e.State = state
			return nil
		}
	}
	return fmt.Errorf("no document modified")
}

type TestDeviceManager struct{}

func (dm TestDeviceManager) SendRequestWithResponse(ctx context.Context, device types.Device, req types.DeviceRequest) (types.DeviceResponse, error) {
	return types.DeviceResponse{
		CorrelationData: req.CorrelationData,
		State:           req.State,
		Status:          "SUCCESS",
	}, nil

}

func TestMain(m *testing.M) {
	f = NewFulfillment(TestDatabase{}, TestDeviceManager{}, opts)
	os.Exit(m.Run())
}
