package server

import (
	"fmt"
	"net/http"
	"os"

	"github.com/gbaranski/houseflow/auth/database"
	"github.com/gbaranski/houseflow/auth/types"
	"github.com/gbaranski/houseflow/auth/utils"
	"github.com/gin-gonic/gin"
	"github.com/gin-gonic/gin/binding"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
)

// AuthQuery sent by google
type AuthQuery struct {
	ClientID     string `form:"client_id" binding:"required"`
	RedirectURI  string `form:"redirect_uri" binding:"required"`
	State        string `form:"state" binding:"required"`
	Scope        string `form:"scope"`
	ResponseType string `form:"response_type" binding:"required"`
	UserLocale   string `form:"user_locale"`
}

// TokenQuery sent by google
type TokenQuery struct {
	ClientID     string `form:"client_id" binding:"required"`
	ClientSecret string `form:"client_secret" binding:"required"`
	GrantType    string `form:"grant_type" binding:"required"`
	Code         string `form:"code" binding:"required_if=GrantType authorization_code"`
	RedirectURI  string `form:"redirect_uri" binding:"required_if=GrantType authorization_code"`
	RefreshToken string `form:"refresh_token" binding:"required_if=GrantType refresh_token"`
}

// ClientIDEnv Enviroment varaible name for ClientID
const ClientIDEnv = "AUTH_CLIENT_ID"

// ClientSecretEnv Enviroment varaible name for ClientSecret
const ClientSecretEnv = "AUTH_CLIENT_SECRET"

// Fill it later
const projectID = "houseflow-prod"

func validateRedirectURI(uri string) bool {
	return uri == fmt.Sprintf("https://oauth-redirect.googleusercontent.com/r/%s", projectID) || uri == fmt.Sprintf("https://oauth-redirect-sandbox.googleusercontent.com/r/%s", projectID)
}

func (s *Server) onAuth(c *gin.Context) {
	var query AuthQuery
	if err := c.MustBindWith(&query, binding.Query); err != nil {
		c.String(http.StatusBadRequest, err.Error())
		return
	}
	if query.ClientID != os.Getenv(ClientIDEnv) {
		c.String(http.StatusBadRequest, "ClientID is invalid")
		return
	}
	if !validateRedirectURI(query.RedirectURI) {
		c.JSON(http.StatusBadRequest, gin.H{
			"error": "redirect_uri is invalid",
		})
	}
	c.HTML(http.StatusOK, "auth.tmpl", gin.H{
		"redirect_uri": query.RedirectURI,
		"state":        query.State,
	})
}

func (s *Server) createRedirectURI(q *AuthQuery, userID primitive.ObjectID) (*string, error) {
	token, err := utils.CreateAuthorizationCode(userID)
	if err != nil {
		return nil, err
	}
	redirectURI := fmt.Sprintf("%s?code=%s&state=%s", q.RedirectURI, token.Token.Raw, q.State)

	return &redirectURI, nil
}

func (s *Server) onLogin(c *gin.Context) {
	var form struct {
		Email    string `form:"email"`
		Password string `form:"password"`
	}
	var query AuthQuery
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
	redirectURI, err := s.createRedirectURI(&query, dbUser.ID)
	if err != nil {
		c.String(http.StatusInternalServerError, err.Error())
		return
	}
	c.Redirect(http.StatusSeeOther, *redirectURI)

}

func (s *Server) onRegister(c *gin.Context) {
	var form types.User
	var query AuthQuery
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
	token, err := utils.VerifyToken(*strtoken, utils.JWTAccessKey)
	if err != nil {
		c.JSON(http.StatusUnauthorized, err.Error())
		return
	}
	_, err = s.db.Redis.DeleteAuth(token.Claims.Id)
	if err != nil {
		c.JSON(http.StatusInternalServerError, err.Error())
		return
	}
	c.JSON(http.StatusOK, "Successfully deleted")
}
