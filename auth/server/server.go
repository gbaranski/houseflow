package server

import (
	"github.com/gin-gonic/gin"

	"github.com/gbaranski/houseflow/auth/database"
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
	s.Router.POST("/login", s.onLogin)
	s.Router.POST("/register", s.onRegister)
	s.Router.POST("/logout", s.onLogout)
	s.Router.POST("/token", s.onToken)
	s.Router.GET("/auth", s.onAuth)
	s.Router.LoadHTMLGlob("template/*")

	return s
}
