package fulfillment

import (
	"context"
	"os"
	"testing"

	"github.com/gbaranski/houseflow/pkg/fulfillment"
	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/google/uuid"
)

const (
	// bcrypt hashed "helloworld"
	helloworld = "$2y$12$sVtI/bYDQ3LWKcGlryQYzeo3IFjIYsl4f4bY6isfBaE3MnaPIcc2e"
)

type userDevice struct {
	UserID   string
	DeviceID string
	Read     bool
	Write    bool
	Execute  bool
}

var (
	opts = Options{
		AccessKey: "someAccessKey",
	}
	f           Fulfillment
	devices     = []types.Device{}
	userDevices = []userDevice{}
	realUser    = types.User{
		ID:        uuid.New().String(),
		FirstName: "John",
		LastName:  "Smith",
		Email:     "john.smith@gmail.com",
		Password:  helloworld,
	}
)

type TestDatabase struct {
}

func (tdb TestDatabase) AddDevice(ctx context.Context, device types.Device) (string, error) {
	id := uuid.New().String()
	device.ID = id
	devices = append(devices, device)
	return id, nil
}

func (tdb TestDatabase) GetDevicesByIDs(ctx context.Context, deviceIDs []string) ([]types.Device, error) {
	found := make([]types.Device, 0)

	for _, e := range deviceIDs {
		for _, v := range devices {
			if e == v.ID {
				found = append(found, v)
				break
			}
		}
	}
	return found, nil
}

func (tdb TestDatabase) GetUserDevicePermissions(ctx context.Context, userID string, deviceID string) (perms types.DevicePermissions, err error) {
	for _, ud := range userDevices {
		if ud.UserID == userID && ud.DeviceID == deviceID {
			return types.DevicePermissions{
				Read:    ud.Read,
				Write:   ud.Write,
				Execute: ud.Execute,
			}, nil
		}
	}
	return types.DevicePermissions{
		Read:    false,
		Write:   false,
		Execute: false,
	}, nil
}

func (tdb TestDatabase) GetDeviceByID(ctx context.Context, deviceID string) (*types.Device, error) {
	for _, device := range devices {
		if device.ID == deviceID {
			return &device, nil
		}
	}
	return nil, nil
}

func (tdb TestDatabase) GetUserDevices(ctx context.Context, userID string) (devices []types.Device, err error) {
	for _, ud := range userDevices {
		if ud.UserID == userID && ud.Read {
			device, err := tdb.GetDeviceByID(ctx, ud.DeviceID)
			if err != nil {
				return nil, err
			}
			if device == nil {
				continue
			}
			devices = append(devices, *device)
		}
	}
	return devices, nil
}

func (tdb TestDatabase) GetUserByID(ctx context.Context, id string) (*types.User, error) {
	if id == realUser.ID {
		return &realUser, nil
	}
	return nil, nil
}

var commands = make(chan string, 1)

type TestDeviceManager struct{}

func (dm TestDeviceManager) SendRequestWithResponse(ctx context.Context, device types.Device, req types.DeviceRequest) (types.DeviceResponse, error) {
	commands <- req.Command
	return types.DeviceResponse{
		CorrelationData: req.CorrelationData,
		State:           req.State,
		Status:          "SUCCESS",
	}, nil
}

func (dm TestDeviceManager) FetchDeviceState(ctx context.Context, deviceID string) (types.DeviceResponse, error) {
	return types.DeviceResponse{
		CorrelationData: "sjsdsklfdksjaasjk",
		State: map[string]interface{}{
			"online": true,
			"on":     true,
		},
		Status: "SUCCESS",
	}, nil

}

func TestMain(m *testing.M) {
	f = NewFulfillment(TestDatabase{}, TestDeviceManager{}, opts)

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
