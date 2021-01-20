package fulfillment

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"io/ioutil"
	"net/http"
	"time"

	"github.com/gbaranski/houseflow/pkg/fulfillment"
	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/gbaranski/houseflow/pkg/utils"
	"github.com/go-chi/chi"
	"github.com/go-chi/chi/middleware"
)

// Options for fulfillment
type Options struct {
	// AccessKey is secret for signing access tokens
	//
	// *Required*
	AccessKey string `env:"ACCESS_KEY,required"`
}

// Database is interface for database
type Database interface {
	AddDevice(ctx context.Context, device types.Device) (string, error)
	GetDeviceByID(ctx context.Context, deviceID string) (*types.Device, error)
	GetDevicesByIDs(ctx context.Context, deviceIDs []string) ([]types.Device, error)

	GetUserDevicePermissions(ctx context.Context, userID string, deviceID string) (perms types.DevicePermissions, err error)
	GetUserDevices(ctx context.Context, userID string) ([]types.Device, error)
	GetUserByID(ctx context.Context, id string) (*types.User, error)
}

// Devmgmt is shortcut for DeviceManager
type Devmgmt interface {
	SendCommand(ctx context.Context, device types.Device, comamnd string, params map[string]interface{}) (types.DeviceResponse, error)
	FetchDeviceState(ctx context.Context, deviceID string) (types.DeviceResponse, error)
}

// Fulfillment hold root server state
type Fulfillment struct {
	Router  *chi.Mux
	devmgmt Devmgmt
	db      Database
	opts    Options
}

// New creates server, it won't run till Server.Start
func New(db Database, devmgmt Devmgmt, opts Options) Fulfillment {
	f := Fulfillment{
		db:      db,
		devmgmt: devmgmt,
		Router:  chi.NewRouter(),
		opts:    opts,
	}
	f.Router.Use(middleware.Logger)
	f.Router.Use(middleware.Recoverer)
	f.Router.Use(middleware.RealIP)
	f.Router.Use(middleware.Heartbeat("/ping"))
	f.Router.Use(middleware.Timeout(time.Second * 10))

	f.Router.Post("/webhook", f.onWebhook)
	f.Router.Get("/addDevice", f.onAddDevice)
	return f
}

// Only for testing purposes
func (f *Fulfillment) onAddDevice(w http.ResponseWriter, r *http.Request) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*3)
	defer cancel()
	f.db.AddDevice(ctx, types.Device{
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
	})

}

type intentRequest struct {
	r    *http.Request
	w    http.ResponseWriter
	base fulfillment.BaseRequest
	user types.User
}

// IntentHandler is type of handler, each of them MUST return something that is json marhsallable
type intentHandler = func(r intentRequest) interface{}

func (f *Fulfillment) getIntentHandler(intent string) (intentHandler, error) {
	switch intent {
	case fulfillment.SyncIntent:
		return f.onSyncIntent, nil
	case fulfillment.QueryIntent:
		return f.onQueryIntent, nil
	case fulfillment.ExecuteIntent:
		return f.onExecuteIntent, nil
	case fulfillment.DisconnectIntent:
		return nil, fmt.Errorf("not implemented yet")
	default:
		return nil, fmt.Errorf("unrecognized intent")
	}
}

func (f *Fulfillment) onWebhook(w http.ResponseWriter, r *http.Request) {
	var (
		bodybuf bytes.Buffer
		base    fulfillment.BaseRequest
	)

	if err := json.NewDecoder(io.TeeReader(r.Body, &bodybuf)).Decode(&base); err != nil {
		json, _ := json.Marshal(types.ResponseError{
			Name:        "fail_parse_json",
			Description: err.Error(),
		})
		w.WriteHeader(http.StatusUnprocessableEntity)
		w.Write(json)
		return
	}

	userID, err := utils.ExtractWithVerifyUserToken(r, []byte(f.opts.AccessKey))
	if err != nil {
		json, _ := json.Marshal(types.ResponseError{
			Name:        "invalid_grant",
			Description: err.Error(),
		})
		w.WriteHeader(http.StatusUnauthorized)
		w.Write(json)
		return
	}

	ctx, cancel := context.WithTimeout(context.Background(), time.Second*3)
	defer cancel()
	user, err := f.db.GetUserByID(ctx, userID)

	if err != nil {
		json, _ := json.Marshal(types.ResponseError{
			Name:        "fail_get_user",
			Description: err.Error(),
		})
		w.WriteHeader(http.StatusInternalServerError)
		w.Write(json)
		return
	}
	if user == nil {
		json, _ := json.Marshal(types.ResponseError{
			Name: "user_not_found",
		})
		w.WriteHeader(http.StatusNotFound)
		w.Write(json)
		return
	}
	handler, err := f.getIntentHandler(base.Inputs[0].Intent)
	if err != nil {
		json, _ := json.Marshal(types.ResponseError{
			Name:        "invalid_intent",
			Description: err.Error(),
		})
		w.WriteHeader(http.StatusBadRequest)
		w.Write(json)
		return
	}
	r.Body = ioutil.NopCloser(&bodybuf)
	res := handler(intentRequest{
		r:    r,
		w:    w,
		base: base,
		user: *user,
	})
	resjson, err := json.Marshal(res)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		json, _ := json.Marshal(types.ResponseError{
			Name:        "fail_marshall_response",
			Description: err.Error(),
		})
		w.Write(json)
		return
	}
	w.Write(resjson)
}
