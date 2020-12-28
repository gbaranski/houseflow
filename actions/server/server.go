package server

import (
	"net/http"

	database "github.com/gbaranski/houseflow/actions/database"
	"github.com/gin-gonic/gin"
)

// Server hold root server state
type Server struct {
	db     *database.Database
	Router *gin.Engine
}

// NewServer creates server, it won't run till Server.Start
func NewServer(db *database.Database) *Server {
	s := &Server{
		db: db,
	}
	s.Router = gin.Default()
	s.Router.GET("/", s.onRoot)
	s.Router.POST("/fulfillment", s.onFulfillment)

	return s
}

func (s *Server) onRoot(c *gin.Context) {
	c.String(http.StatusOK, "Hello world")

}
