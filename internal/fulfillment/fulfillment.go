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
	"github.com/gbaranski/houseflow/pkg/token"
	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/gbaranski/houseflow/pkg/utils"
	"github.com/go-chi/chi"
	"github.com/go-chi/chi/middleware"
	"github.com/sirupsen/logrus"
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
	FetchDeviceState(ctx context.Context, device types.Device) (types.DeviceResponse, error)
	SendActionCommand(
		ctx context.Context,
		device types.Device,
		command string,
		params map[string]interface{},
	) (types.DeviceResponse, error)
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
	device := types.Device{
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
			DeviceInfo: fulfillment.DeviceInfo{
				Manufacturer: "gbaranski's garage",
				Model:        "Nightlamp",
				HwVersion:    "1.0.0",
				SwVersion:    "0.1.1",
			},
		},
	}
	f.db.AddDevice(r.Context(), device)

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
		logrus.Errorf("fail decoding body %s\n", err.Error())
		utils.ReturnError(w, types.ResponseError{
			Name:        "invalid_json",
			Description: err.Error(),
			StatusCode:  http.StatusUnprocessableEntity,
		})
		return
	}

	signedTokenBase64 := token.ExtractHeaderToken(r)
	if signedTokenBase64 == nil {
		logrus.Errorf("missing token in headers\n")
		utils.ReturnError(w, types.ResponseError{
			Name:       "missing_token",
			StatusCode: http.StatusBadRequest,
		})
		return
	}
	signedToken, err := token.NewSignedFromBase64WithVerify([]byte(f.opts.AccessKey), []byte(*signedTokenBase64))
	if err != nil {
		logrus.Errorf("fail decoding signed token %s\n", err.Error())
		utils.ReturnError(w, types.ResponseError{
			Name:        "invalid_grant",
			Description: err.Error(),
			StatusCode:  http.StatusForbidden,
		})
		return
	}

	userID := signedToken.Parse().Audience
	user, err := f.db.GetUserByID(r.Context(), string(userID))
	if err != nil {
		logrus.Errorf("fail retrieving user from database %s\n", err.Error())
		utils.ReturnError(w, types.ResponseError{
			Name:        "fail_get_user",
			Description: err.Error(),
			StatusCode:  http.StatusInternalServerError,
		})
		return
	}
	if user == nil {
		logrus.Errorf("user not found in database\n")
		utils.ReturnError(w, types.ResponseError{
			Name:       "user_not_found",
			StatusCode: http.StatusNotFound,
		})
		return
	}
	handler, err := f.getIntentHandler(base.Inputs[0].Intent)
	if err != nil {
		logrus.Errorf("invalid intent\n")
		utils.ReturnError(w, types.ResponseError{
			Name:        "invalid_intent",
			Description: err.Error(),
			StatusCode:  http.StatusBadRequest,
		})
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
		logrus.Errorf("fail marshalling response %s\n", err.Error())
		utils.ReturnError(w, types.ResponseError{
			Name:        "fail_marshall_response",
			Description: err.Error(),
			StatusCode:  http.StatusInternalServerError,
		})
		return
	}
	w.Write(resjson)
}
