package actions

import (
	"context"
	"net/http"
	"time"

	"github.com/gbaranski/houseflow/pkg/database"
	"github.com/gbaranski/houseflow/pkg/fulfillment"
	"github.com/gbaranski/houseflow/pkg/mqtt"
	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/gbaranski/houseflow/pkg/utils"
	"github.com/gin-gonic/gin"
	"github.com/gin-gonic/gin/binding"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
)

var (
	AccessKey = utils.MustGetEnv("ACCESS_KEY")
)

// Server hold root server state
type Server struct {
	mongo  database.Mongo
	Router *gin.Engine
	mqtt   mqtt.MQTT
}

// NewServer creates server, it won't run till Server.Start
func NewServer(mongo database.Mongo, mqtt mqtt.MQTT) *Server {
	s := &Server{
		mongo:  mongo,
		Router: gin.Default(),
		mqtt:   mqtt,
	}
	s.Router.Use(utils.LogResponseBody)
	s.Router.POST("/fulfillment", s.onFulfillment)
	return s
}

func (s *Server) redirectIntent(c *gin.Context, intent string, user types.User, userDevices []types.Device) {
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
		s.onSync(c, req, user, userDevices)
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
		s.OnQuery(c, req, user, userDevices)
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
		s.onExecute(c, er, user, userDevices)
	case fulfillment.DisconnectIntent:
		c.JSON(http.StatusNotImplemented, gin.H{
			"error": "not_implemented",
		})
	}

}

func (s *Server) onFulfillment(c *gin.Context) {
	var base fulfillment.BaseRequest

	if err := c.ShouldBindBodyWith(&base, binding.JSON); err != nil {
		c.JSON(http.StatusUnprocessableEntity, gin.H{
			"error":             "init_parse_invalid_json",
			"error_description": err.Error(),
		})
		return
	}

	userID, err := utils.ExtractWithVerifyUserToken(c.Request, []byte(AccessKey))
	if err != nil {
		c.JSON(http.StatusUnauthorized, gin.H{
			"error":             "invalid_token",
			"error_description": err.Error(),
		})
		return
	}

	ctx, cancel := context.WithTimeout(context.Background(), time.Second*3)
	defer cancel()
	user, err := s.mongo.GetUserByID(ctx, *userID)

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

	var deviceIDs []primitive.ObjectID
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
	userDevices, err := s.mongo.GetDevicesByIDs(ctx, deviceIDs)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"error":             "get_devices_fail",
			"error_description": err.Error(),
		})
		return
	}
	s.redirectIntent(c, base.Inputs[0].Intent, user, userDevices)
}
