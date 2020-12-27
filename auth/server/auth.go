package server

import (
	"fmt"
	"net/http"

	"github.com/gbaranski/houseflow/auth/database"
	"github.com/gbaranski/houseflow/auth/types"
	"github.com/gbaranski/houseflow/auth/utils"
	"github.com/gin-gonic/gin"
	"github.com/gin-gonic/gin/binding"
	"go.mongodb.org/mongo-driver/mongo"
)

// OAuth2Query sddas
type OAuth2Query struct {
	ClientID     string `form:"client_id" binding:"required"`
	RedirectURI  string `form:"redirect_uri" binding:"required"`
	State        string `form:"state" binding:"required"`
	Scope        string `form:"scope"`
	ResponseType string `form:"response_type" binding:"required"`
	UserLocale   string `form:"user_locale"`
}

func (s *Server) onAuth(c *gin.Context) {
	var query OAuth2Query
	if err := c.MustBindWith(&query, binding.Query); err != nil {
		c.String(http.StatusBadRequest, err.Error())
		return
	}
	if query.ClientID != clientID {
		c.String(http.StatusBadRequest, "ClientID is invalid")
		return
	}
	if query.RedirectURI != fmt.Sprintf("https://oauth-redirect.googleusercontent.com/r/%s", projectID) &&
		query.RedirectURI != fmt.Sprintf("https://oauth-redirect-sandbox.googleusercontent.com/r/%s", projectID) {
		c.String(http.StatusBadRequest, "RedirectURI is invalid")
		return
	}
	c.HTML(http.StatusOK, "auth.tmpl", gin.H{
		"redirect_uri": query.RedirectURI,
		"state":        query.State,
	})
}

func (s *Server) onLogin(c *gin.Context) {
	var form struct {
		Email    string `form:"email"`
		Password string `form:"password"`
	}
	var query OAuth2Query
	if err := c.MustBindWith(&query, binding.Query); err != nil {
		c.String(http.StatusBadRequest, err.Error())
		return
	}
	if err := c.MustBindWith(&form, binding.FormPost); err != nil {
		c.String(http.StatusUnprocessableEntity, err.Error())
		return
	}
	dbUser, err := s.db.Mongo.GetUser(form.Email)
	if err != nil {
		if err == mongo.ErrNoDocuments {
			c.JSON(http.StatusUnauthorized, "Invalid email or password")
			return
		}
		c.JSON(http.StatusInternalServerError, err.Error())
		return
	}

	//compare the user from the request, with the one we defined:
	if dbUser.Email != form.Email {
		c.JSON(http.StatusUnauthorized, "Invalid email or password")
		return
	}
	passmatch := utils.CheckPasswordHash(form.Password, dbUser.Password)
	if !passmatch {
		c.JSON(http.StatusUnauthorized, "Invalid email or password")
		return
	}

	tokens, err := utils.CreateTokens()
	if err != nil {
		c.JSON(http.StatusInternalServerError, err.Error())
		return
	}
	err = s.db.Redis.CreateAuth(dbUser.ID, tokens)
	if err != nil {
		c.JSON(http.StatusInternalServerError, err.Error())
		return
	}
	token := gin.H{
		"access_token":  tokens.AccessToken.Token,
		"refresh_token": tokens.RefreshToken.Token,
	}
	c.JSON(http.StatusOK, token)
}

func (s *Server) onRegister(c *gin.Context) {
	var form types.User
	var query OAuth2Query
	if err := c.MustBindWith(&query, binding.Query); err != nil {
		c.String(http.StatusBadRequest, err.Error())
		return
	}
	if err := c.MustBindWith(&form, binding.FormPost); err != nil {
		c.String(http.StatusUnprocessableEntity, err.Error())
		return
	}
	id, err := s.db.Mongo.AddUser(form)
	if err != nil {
		if database.IsDuplicateError(err) {
			c.String(http.StatusConflict, "Account already exists")
			return
		}
		c.JSON(http.StatusUnprocessableEntity, err.Error())
		return
	}
	c.String(http.StatusOK, fmt.Sprintf("Account created with ID: %s, now you can log in", id.Hex()))
}

func (s *Server) onLogout(c *gin.Context) {
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
	_, err = s.db.Redis.DeleteAuth(claims.Id)
	if err != nil {
		c.JSON(http.StatusInternalServerError, err.Error())
		return
	}
	c.JSON(http.StatusOK, "Successfully deleted")
}
