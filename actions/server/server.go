package server

import (
	"bytes"
	"context"
	"fmt"
	"net/http"
	"time"

	"github.com/gbaranski/houseflow/pkg/fulfillment"
	"github.com/gbaranski/houseflow/pkg/database"
	"github.com/gbaranski/houseflow/pkg/mqtt"
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
	mongo     *database.Mongo
	Router *gin.Engine
  mqtt   mqtt.MQTT
}

type responseBodyWriter struct {
	gin.ResponseWriter
	body *bytes.Buffer
}

func (r responseBodyWriter) Write(b []byte) (int, error) {
	r.body.Write(b)
	return r.ResponseWriter.Write(b)
}

func logResponseBody(c *gin.Context) {
	w := &responseBodyWriter{body: &bytes.Buffer{}, ResponseWriter: c.Writer}
	c.Writer = w
	c.Next()
	fmt.Println("Response body: " + w.body.String())
}

// NewServer creates server, it won't run till Server.Start
func NewServer(mongo *database.Mongo, mqtt mqtt.MQTT) *Server {
	s := &Server{
		mongo:  mongo,
		Router: gin.Default(),
		mqtt:   mqtt,
	}
	s.Router.Use(logResponseBody)
	s.Router.POST("/fulfillment", s.onFulfillment)
	return s
}

func (s *Server) onFulfillment(c *gin.Context) {
	var base fulfillment.BaseRequest

	if err := c.ShouldBindBodyWith(&base, binding.JSON); err != nil {
		fmt.Println("Init parse failed ", err.Error())
		c.JSON(http.StatusUnprocessableEntity, gin.H{
			"error":             "init_parse_invalid_json",
			"error_description": err.Error(),
		})
		return
	}

	token := utils.ExtractHeaderToken(c.Request)
	if token == nil {
		c.JSON(http.StatusUnauthorized, gin.H{
			"error": "missing_bearer_token",
		})
		return
	}
	td, err := utils.VerifyToken(*token, []byte(AccessKey))
	if err != nil {
		c.JSON(http.StatusUnauthorized, gin.H{
			"error":             "invalid_token",
			"error_description": err.Error(),
		})
		return
	}
	userID, err := primitive.ObjectIDFromHex(td.Audience)
	if err != nil {
		c.JSON(http.StatusUnauthorized, gin.H{
			"error":             "invalid_token_aud",
			"error_description": err.Error(),
		})
		return
	}

  ctx, cancel := context.WithTimeout(context.Background(), time.Second * 3)
  defer cancel()

	user, err := s.mongo.GetUserByID(ctx, userID)
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



	fmt.Printf("Base request: %+v\n", base)
	// Currently not even expecting more than 1 input
	switch base.Inputs[0].Intent {
	case fulfillment.SyncIntent:
		var sr fulfillment.SyncRequest
		err = c.ShouldBindBodyWith(&sr, binding.JSON)
		if err != nil {
			c.JSON(http.StatusBadRequest, gin.H{
				"error":             "sync_invalid_json",
				"error_description": err.Error(),
			})
			return
		}
		s.onSync(c, sr, user, userDevices)
	case fulfillment.QueryIntent:
		var qr fulfillment.QueryRequest
		err = c.ShouldBindBodyWith(&qr, binding.JSON)
		if err != nil {
			c.JSON(http.StatusBadRequest, gin.H{
				"error":             "query_invalid_json",
				"error_description": err.Error(),
			})
			return
		}
		s.OnQuery(c, qr, user, userDevices)
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

