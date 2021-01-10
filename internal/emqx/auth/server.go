package auth

import (
	"fmt"

	"github.com/gbaranski/houseflow/pkg/database"
	"github.com/gin-gonic/gin"
)

// Server holds state
type Server struct {
	mongo     database.Mongo
	Router *gin.Engine
}

// NewServer implements server
func NewServer(mongo database.Mongo) Server {
	s := Server{
		mongo:     mongo,
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
