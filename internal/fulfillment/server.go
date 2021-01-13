package fulfillment

import (
	"context"
	"net/http"
	"time"

	"github.com/gbaranski/houseflow/pkg/fulfillment"
	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/gbaranski/houseflow/pkg/utils"
	"github.com/gin-gonic/gin"
	"github.com/gin-gonic/gin/binding"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
)

// Options for fulfillment
type Options struct {
	// AccessKey is secret for signing access tokens
	//
	// *Required*
	AccessKey string
}

// Database is interface for database
type Database interface {
	AddDevice(ctx context.Context, device types.Device) (primitive.ObjectID, error)
	GetUserByID(ctx context.Context, id primitive.ObjectID) (types.User, error)
	GetDevicesByIDs(ctx context.Context, deviceIDs []primitive.ObjectID) ([]types.Device, error)
	UpdateDeviceState(ctx context.Context, deviceID primitive.ObjectID, state map[string]interface{}) error
}

// DeviceManager is interface
type DeviceManager interface {
	SendRequestWithResponse(ctx context.Context, device types.Device, req types.DeviceRequest) (types.DeviceResponse, error)
}

// Fulfillment hold root server state
type Fulfillment struct {
	Router *gin.Engine
	dm     DeviceManager
	db     Database
	opts   Options
}

// NewFulfillment creates server, it won't run till Server.Start
func NewFulfillment(db Database, dm DeviceManager, opts Options) *Fulfillment {
	f := &Fulfillment{
		db:     db,
		dm:     dm,
		Router: gin.Default(),
		opts:   opts,
	}
	f.Router.Use(utils.LogResponseBody)
	f.Router.POST("/fulfillment", f.onFulfillment)
	f.Router.GET("/addDevice", f.onAddDevice)
	return f
}

// Only for testing purposes
func (f *Fulfillment) onAddDevice(c *gin.Context) {
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

func (f *Fulfillment) redirectIntent(c *gin.Context, intent string, user types.User, userDevices []types.Device) {
	switch intent {
	case fulfillment.SyncIntent:
		var req fulfillment.SyncRequest
		err := c.ShouldBindBodyWith(&req, binding.JSON)
		if err != nil {
			c.JSON(http.StatusBadRequest, gin.H{
				"error":             "sync_invalid_json",
				"error_description": err.Error(),
			})
			return
		}
		f.onSync(c, req, user, userDevices)
	case fulfillment.QueryIntent:
		var req fulfillment.QueryRequest
		err := c.ShouldBindBodyWith(&req, binding.JSON)
		if err != nil {
			c.JSON(http.StatusBadRequest, gin.H{
				"error":             "query_invalid_json",
				"error_description": err.Error(),
			})
			return
		}
		f.onQuery(c, req, user, userDevices)
	case fulfillment.ExecuteIntent:
		var er fulfillment.ExecuteRequest
		err := c.ShouldBindBodyWith(&er, binding.JSON)
		if err != nil {
			c.JSON(http.StatusBadRequest, gin.H{
				"error":             "execute_invalid_json",
				"error_description": err.Error(),
			})
			return
		}
		f.onExecute(c, er, user, userDevices)
	case fulfillment.DisconnectIntent:
		c.JSON(http.StatusNotImplemented, gin.H{
			"error": "not_implemented",
		})
	}

}

func (f *Fulfillment) onFulfillment(c *gin.Context) {
	var base fulfillment.BaseRequest

	if err := c.ShouldBindBodyWith(&base, binding.JSON); err != nil {
		c.JSON(http.StatusUnprocessableEntity, gin.H{
			"error":             "init_parse_invalid_json",
			"error_description": err.Error(),
		})
		return
	}

	userID, err := utils.ExtractWithVerifyUserToken(c.Request, []byte(f.opts.AccessKey))
	if err != nil {
		c.JSON(http.StatusUnauthorized, gin.H{
			"error":             "invalid_token",
			"error_description": err.Error(),
		})
		return
	}

	ctx, cancel := context.WithTimeout(context.Background(), time.Second*3)
	defer cancel()
	user, err := f.db.GetUserByID(ctx, *userID)

	if err != nil {
		if err == mongo.ErrNoDocuments {
			c.JSON(http.StatusNotFound, gin.H{
				"error":             "user_not_found",
				"error_description": err.Error(),
			})
		} else {
			c.JSON(http.StatusInternalServerError, gin.H{
				"error":             "unable_retreive_user",
				"error_description": err.Error(),
			})
		}
		return
	}

	deviceIDs := make([]primitive.ObjectID, len(user.Devices))
	for _, id := range user.Devices {
		objID, err := primitive.ObjectIDFromHex(id)
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{
				"error":             "convert_object_id_fail",
				"error_description": err.Error(),
			})
			return
		}
		deviceIDs = append(deviceIDs, objID)
	}
	userDevices, err := f.db.GetDevicesByIDs(ctx, deviceIDs)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"error":             "get_devices_fail",
			"error_description": err.Error(),
		})
		return
	}
	f.redirectIntent(c, base.Inputs[0].Intent, user, userDevices)
}
