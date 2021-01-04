package server

import (
	"encoding/json"
	"fmt"

	"github.com/gbaranski/houseflow/actions/database"
	"github.com/gbaranski/houseflow/actions/fulfillment"
	"github.com/gin-gonic/gin"
)

// Server hold root server state
type Server struct {
	db          *database.Database
	Router      *gin.Engine
	fulfillment fulfillment.Fulfillment
}

// NewServer creates server, it won't run till Server.Start
func NewServer(db *database.Database) *Server {
	s := &Server{
		db: db,
	}
	s.Router = gin.Default()
	s.Router.POST("/fulfillment")
	return s
}

type test1 struct {
	Something1 string `json:"something1"`
}

type test2 struct {
	Something1 string `json:"something1"`
	Something2 string `json:"something2"`
}

func (s *Server) onFulfillment(c *gin.Context) {
	jsonstr := `{"something1":"smth1","something2":"smth2"}`
	var test1 test1
	_ = json.Unmarshal([]byte(jsonstr), &test1)

	var test2 test2
	_ = json.Unmarshal([]byte(jsonstr), &test2)

	fmt.Printf("Test1: %+v\n", test1)
	fmt.Printf("Test2: %+v\n", test2)

}
