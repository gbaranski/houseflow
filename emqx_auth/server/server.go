package server

import (
	"github.com/gbaranski/houseflow/emqx_auth/database"
	"github.com/gin-gonic/gin"
)

type Server struct {
	db     *database.Database
	Router *gin.Engine
}

func NewServer(db *database.Database) Server {
	s := Server{
		db:     db,
		Router: gin.Default()}

	s.Router.POST("/user", s.onUser)
	s.Router.POST("/acl", s.onACL)
	return s
}

func (s *Server) onUser(c *gin.Context) {

}

func (s *Server) onACL(c *gin.Context) {

}
