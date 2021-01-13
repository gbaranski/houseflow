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
var db = TestDatabase{}
var dm = TestDeviceManager{}
var realUser = types.User{
	ID:        primitive.NewObjectID(),
	FirstName: "John",
	LastName:  "Smith",
	Email:     "john.smith@gmail.com",
	Password:  helloworld,
	Devices:   []string{},
}

type TestDatabase struct {
	Devices []types.Device
}

func (tdb TestDatabase) AddDevice(ctx context.Context, device types.Device) (primitive.ObjectID, error) {
	device.ID = primitive.NewObjectID()
	db.Devices = append(db.Devices, device)

	return device.ID, nil
}

func (tdb TestDatabase) GetUserByID(ctx context.Context, id primitive.ObjectID) (types.User, error) {
	if id == realUser.ID {
		return realUser, nil
	}
	return types.User{}, mongo.ErrNoDocuments
}
func (tdb TestDatabase) GetDevicesByIDs(ctx context.Context, deviceIDs []primitive.ObjectID) ([]types.Device, error) {
	found := make([]types.Device, 0)

	for _, e := range deviceIDs {
		for _, v := range db.Devices {
			if e == v.ID {
				fmt.Println("Found for ", e.Hex())
				found = append(found, v)
				break
			}
		}
	}
	return found, nil
}

func (tdb TestDatabase) UpdateDeviceState(ctx context.Context, deviceID primitive.ObjectID, state map[string]interface{}) error {
	for _, e := range db.Devices {
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
	f = NewFulfillment(db, dm, opts)

	f.db.AddDevice(context.Background(), types.Device{
		Device: fulfillment.Device{
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
	})
	f.db.AddDevice(context.Background(), types.Device{
		Device: fulfillment.Device{
			Type: "action.devices.types.GATE",
			Traits: []string{
				"action.devices.traits.OnOff",
			},
			Name: fulfillment.DeviceName{
				Name: "Gate",
				DefaultNames: []string{
					"Gate",
				},
				Nicknames: []string{
					"Gate",
				},
			},
			WillReportState: true,
			RoomHint:        "Garage",
			DeviceInfo: &fulfillment.DeviceInfo{
				Manufacturer: "gbaranski's garage",
				Model:        "Gate",
				HwVersion:    "1.0.0",
				SwVersion:    "0.1.1",
			},
		},
		PublicKey: "jPcGACNcypUyO9T+lR3Y49s9JpxEuKS0/PMtC7g8AuU=",
		State: map[string]interface{}{
			"online": true,
			"on":     false,
		},
	})
	os.Exit(m.Run())
}
