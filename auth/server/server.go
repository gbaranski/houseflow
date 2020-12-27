package server

import (
	"net/http"

	"github.com/gin-gonic/gin"
	"go.mongodb.org/mongo-driver/bson/primitive"

	"github.com/gbaranski/houseflow/auth/utils"

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
	s.Router.POST("/refresh", s.onRefresh)
	s.Router.POST("/token", s.onToken)
	s.Router.GET("/auth", s.onAuth)
	s.Router.LoadHTMLGlob("template/*")

	return s
}

func (s *Server) onRefresh(c *gin.Context) {
	var reqTokens struct {
		RefreshToken string `json:"refresh_token"`
	}
	if err := c.ShouldBindJSON(&reqTokens); err != nil {
		c.JSON(http.StatusUnprocessableEntity, err.Error())
		return
	}
	token, err := utils.VerifyToken(reqTokens.RefreshToken, utils.JWTRefreshKey)
	if err != nil {
		c.JSON(http.StatusForbidden, err.Error())
		return
	}
	userID, err := s.db.Redis.FetchToken(token.Claims)
	if err != nil {
		c.JSON(http.StatusUnauthorized, err.Error())
		return
	}
	mongoUserID, err := primitive.ObjectIDFromHex(*userID)
	if err != nil {
		c.JSON(http.StatusForbidden, err.Error())
		return
	}
	// Delete old refresh token
	if _, err := s.db.Redis.DeleteAuth(token.Claims.Id); err != nil {
		c.JSON(http.StatusForbidden, err.Error())
		return
	}
	tokens, err := utils.CreateTokenPair()
	if err != nil {
		c.JSON(http.StatusInternalServerError, err.Error())
		return
	}
	if err = s.db.Redis.AddTokenPair(mongoUserID, tokens); err != nil {
		c.JSON(http.StatusInternalServerError, err.Error())
		return
	}
	resTokens := gin.H{
		"access_token":  tokens.AccessToken.Token.Raw,
		"refresh_token": tokens.RefreshToken.Token.Raw,
	}
	c.JSON(http.StatusOK, resTokens)
}
