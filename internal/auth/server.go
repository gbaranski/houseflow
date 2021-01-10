package auth

import (
	"github.com/gin-gonic/gin"

	"github.com/gbaranski/houseflow/pkg/database"
	"github.com/gbaranski/houseflow/pkg/utils"
)

var (
	accessKey            = utils.MustGetEnv("ACCESS_KEY")
	authorizationCodeKey = utils.MustGetEnv("AUTHORIZATION_CODE_KEY")
	refreshKey           = utils.MustGetEnv("REFRESH_KEY")
)

// Server hold root server state
type Server struct {
	mongo  database.Mongo
	redis  database.Redis
	Router *gin.Engine
}

// NewServer creates server, it won't run till Server.Start
func NewServer(mongo database.Mongo, redis database.Redis) *Server {
	s := &Server{
		mongo: mongo,
    redis: redis,
    Router: gin.Default(),
	}
	s.Router.POST("/login", s.onLogin)
	s.Router.POST("/register", s.onRegister)
	s.Router.POST("/logout", s.onLogout)
	s.Router.POST("/token", s.onToken)
	s.Router.GET("/auth", s.onAuth)
	s.Router.LoadHTMLGlob("template/*")

	return s
}
