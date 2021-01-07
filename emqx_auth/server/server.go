package server

import (
	"fmt"

	"github.com/gbaranski/houseflow/emqx_auth/database"
	"github.com/gin-gonic/gin"
)

// Server holds state
type Server struct {
	db     *database.Database
	Router *gin.Engine
}

// NewServer implements server
func NewServer(db *database.Database) Server {
	s := Server{
		db:     db,
		Router: gin.Default()}

	s.Router.POST("/user", s.onUser)
	s.Router.POST("/acl", s.onACL)
	return s
}

func (s *Server) onUser(c *gin.Context) {
	fmt.Println(c.ContentType())
	var r UserRequest
	err := c.Bind(&r)
	if err != nil {
		panic(err)
	}
	fmt.Printf("User request: %+v\n", r)
	c.Status(200)
}

func (s *Server) onACL(c *gin.Context) {
	c.Status(200)
}
