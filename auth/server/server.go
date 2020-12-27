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
	s.Router.POST("/someAction", s.onSomeAction)
	s.Router.GET("/auth", s.onAuth)
	s.Router.LoadHTMLGlob("template/*")

	return s
}

// Move both clientID and projectID to .env
const clientID = "abcdefg"

// Fill it later
const projectID = "houseflow-prod"

func (s *Server) onRefresh(c *gin.Context) {
	var reqTokens struct {
		RefreshToken string `json:"refresh_token"`
	}
	if err := c.ShouldBindJSON(&reqTokens); err != nil {
		c.JSON(http.StatusUnprocessableEntity, err.Error())
		return
	}
	_, claims, err := utils.VerifyToken(reqTokens.RefreshToken, utils.JWTRefreshSecretEnv)
	if err != nil {
		c.JSON(http.StatusForbidden, err.Error())
		return
	}
	userID, err := s.db.Redis.FetchAuth(claims)
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
	if _, err := s.db.Redis.DeleteAuth(claims.Id); err != nil {
		c.JSON(http.StatusForbidden, err.Error())
		return
	}
	tokens, err := utils.CreateTokens()
	if err != nil {
		c.JSON(http.StatusInternalServerError, err.Error())
		return
	}
	if err = s.db.Redis.CreateAuth(mongoUserID, tokens); err != nil {
		c.JSON(http.StatusInternalServerError, err.Error())
		return
	}
	resTokens := gin.H{
		"access_token":  tokens.AccessToken.Token,
		"refresh_token": tokens.RefreshToken.Token,
	}
	c.JSON(http.StatusOK, resTokens)
}

func (s *Server) onSomeAction(c *gin.Context) {
	strtoken := utils.ExtractToken(c.Request)
	if strtoken == nil {
		c.JSON(http.StatusUnauthorized, "Authorization token not provided")
		return
	}
	_, claims, err := utils.VerifyToken(*strtoken, utils.JWTAccessSecretEnv)
	if err != nil {
		c.JSON(http.StatusUnauthorized, err.Error())
		return
	}
	userID, err := s.db.Redis.FetchAuth(claims)
	if err != nil {
		c.JSON(http.StatusUnauthorized, err.Error())
		return
	}

	c.JSON(http.StatusOK, gin.H{
		"expires": claims.ExpiresAt,
		"ID":      claims.Id,
		"userID":  userID,
	})
}
