package server

import (
	"bytes"
	"fmt"
	"net/http"
	"strings"

	mqtt "github.com/eclipse/paho.mqtt.golang"
	"github.com/gbaranski/houseflow/actions/config"
	"github.com/gbaranski/houseflow/actions/database"
	"github.com/gbaranski/houseflow/actions/fulfillment"
	"github.com/gbaranski/houseflow/actions/utils"
	"github.com/gin-gonic/gin"
	"github.com/gin-gonic/gin/binding"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
)

// Server hold root server state
type Server struct {
	db     *database.Database
	Router *gin.Engine
	mqtt   mqtt.Client
	config config.Config
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
func NewServer(db *database.Database, mqtt mqtt.Client, config config.Config) *Server {
	s := &Server{
		db:     db,
		Router: gin.Default(),
		mqtt:   mqtt,
		config: config,
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

	token := extractToken(c.Request)
	if token == nil {
		c.JSON(http.StatusUnauthorized, gin.H{
			"error": "missing_bearer_token",
		})
		return
	}
	td, err := utils.VerifyToken(*token, utils.JWTAccessKey)
	if err != nil {
		c.JSON(http.StatusUnauthorized, gin.H{
			"error":             "invalid_token",
			"error_description": err.Error(),
		})
		return
	}
	userID, err := primitive.ObjectIDFromHex(td.Claims.Audience)
	if err != nil {
		c.JSON(http.StatusUnauthorized, gin.H{
			"error":             "invalid_token_aud",
			"error_description": err.Error(),
		})
		return
	}

	user, err := s.db.Mongo.GetUserByID(userID)
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
		s.onSync(c, sr, *user)
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
		s.OnQuery(c, qr, *user)
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
		s.onExecute(c, er, *user)
	case fulfillment.DisconnectIntent:
		c.JSON(http.StatusNotImplemented, gin.H{
			"error": "not_implemented",
		})
	}

}

func extractToken(r *http.Request) *string {
	bearToken := r.Header.Get("Authorization")
	//normally Authorization the_token_xxx
	strArr := strings.Split(bearToken, " ")
	if len(strArr) == 2 {
		return &strArr[1]
	}
	return nil
}
