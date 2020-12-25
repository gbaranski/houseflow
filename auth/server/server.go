package server

import (
	"log"
	"net/http"

	"github.com/gin-gonic/gin"
	"go.mongodb.org/mongo-driver/mongo"

	"github.com/gbaranski/houseflow/auth/types"
	"github.com/gbaranski/houseflow/auth/utils"

	"github.com/gbaranski/houseflow/auth/database"
)

// Server hold root server state
type Server struct {
	db     *database.Database
	router *gin.Engine
}

// NewServer creates server, it won't run till Server.Start
func NewServer(db *database.Database) *Server {
	return &Server{
		db:     db,
		router: gin.Default(),
	}
}

// Start starts server, this function is blocking
func (s *Server) Start() error {
	log.Println("Starting server at port 8080")
	s.router.POST("/login", s.onLogin)
	s.router.POST("/register", s.onRegister)
	return s.router.Run(":8080")
}

func (s *Server) onLogin(c *gin.Context) {
	var user struct {
		Email    string `json:"email"`
		Password string `json:"password"`
	}
	if err := c.ShouldBindJSON(&user); err != nil {
		c.JSON(http.StatusUnprocessableEntity, "Invalid json provided")
		return
	}
	dbUser, err := s.db.Mongo.GetUser(user.Email)
	if err != nil {
		if err == mongo.ErrNoDocuments {
			c.JSON(http.StatusUnauthorized, "Invalid email or password")
			return
		}
		c.JSON(http.StatusInternalServerError, err.Error())
		return
	}

	//compare the user from the request, with the one we defined:
	if dbUser.Email != user.Email {
		c.JSON(http.StatusUnauthorized, "Invalid email or password")
		return
	}
	passmatch := utils.CheckPasswordHash(user.Password, dbUser.Password)
	if !passmatch {
		c.JSON(http.StatusUnauthorized, "Invalid email or password")
		return
	}

	token, err := utils.CreateToken(dbUser.ID)
	if err != nil {
		c.JSON(http.StatusUnprocessableEntity, err.Error())
		return
	}
	err = s.db.Redis.CreateAuth(dbUser.ID, token)
	if err != nil {
		c.JSON(http.StatusUnprocessableEntity, err.Error())
		return
	}
	tokens := map[string]string{
		"access_token":  token.AccessToken.Token,
		"refresh_token": token.RefreshToken.Token,
	}
	c.JSON(http.StatusOK, tokens)
}

func (s *Server) onRegister(c *gin.Context) {
	var user types.User
	if err := c.ShouldBindJSON(&user); err != nil {
		c.JSON(http.StatusUnprocessableEntity, "Invalid json provided")
		return
	}
	if err := user.Validate(); err != nil {
		c.JSON(http.StatusBadRequest, err.Error())
		return
	}
	id, err := s.db.Mongo.AddUser(user)
	if err != nil {
		if database.IsDuplicateError(err) {
			c.JSON(http.StatusConflict, "Account already exists")
			return
		}
		c.JSON(http.StatusUnprocessableEntity, err.Error())
		return
	}
	c.JSON(http.StatusOK, map[string]string{
		"id": id.String(),
	})
}
