package auth

import (
	"context"
	"fmt"
	"net/http"
	"time"

	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/gbaranski/houseflow/pkg/utils"
	"github.com/gin-gonic/gin"
	"github.com/gin-gonic/gin/binding"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
)

// LoginPageQuery sent by google
type LoginPageQuery struct {
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

var (
	projectID    = utils.MustGetEnv("PROJECT_ID")
	clientID     = utils.MustGetEnv("OAUTH_CLIENT_ID")
	clientSecret = utils.MustGetEnv("OAUTH_CLIENT_SECRET")
)

func validateRedirectURI(uri string) bool {
	return uri == fmt.Sprintf("https://oauth-redirect.googleusercontent.com/r/%s", projectID) || uri == fmt.Sprintf("https://oauth-redirect-sandbox.googleusercontent.com/r/%s", projectID)
}

func (s *Server) onLoginPage(c *gin.Context) {
	var query LoginPageQuery
	if err := c.MustBindWith(&query, binding.Query); err != nil {
		c.String(http.StatusBadRequest, err.Error())
		return
	}
	if query.ClientID != clientID {
		c.String(http.StatusBadRequest, "ClientID is invalid")
		return
	}
	if !validateRedirectURI(query.RedirectURI) {
		c.JSON(http.StatusBadRequest, gin.H{
			"error": "redirect_uri is invalid",
		})
		return
	}
	c.HTML(http.StatusOK, "auth.tmpl", gin.H{
		"redirect_uri": query.RedirectURI,
		"state":        query.State,
	})
}

func (s *Server) createRedirectURI(q *LoginPageQuery, userID primitive.ObjectID) (string, error) {
	token := utils.Token{
		Audience:  userID.Hex(),
		ExpiresAt: time.Now().Add(utils.AuthorizationCodeDuration).Unix(),
	}
	strtoken, err := token.Sign([]byte(authorizationCodeKey))
	if err != nil {
		return "", err
	}

	return fmt.Sprintf("%s?code=%s&state=%s", q.RedirectURI, strtoken, q.State), nil
}

func (s *Server) onLogin(c *gin.Context) {
	var form struct {
		Email    string `form:"email"`
		Password string `form:"password"`
	}
	var query LoginPageQuery
	if err := c.MustBindWith(&query, binding.Query); err != nil {
		c.String(http.StatusBadRequest, err.Error())
		return
	}
	if err := c.MustBindWith(&form, binding.FormPost); err != nil {
		c.String(http.StatusUnprocessableEntity, err.Error())
		return
	}
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*3)
	defer cancel()
	dbUser, err := s.mongo.GetUserByEmail(ctx, form.Email)
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
	passmatch := utils.ComparePasswordAndHash([]byte(form.Password), []byte(dbUser.Password))
	if !passmatch {
		c.JSON(http.StatusUnauthorized, "Invalid email or password")
		return
	}
	redirectURI, err := s.createRedirectURI(&query, dbUser.ID)
	if err != nil {
		c.String(http.StatusInternalServerError, err.Error())
		return
	}
	c.Redirect(http.StatusSeeOther, redirectURI)

}

func (s *Server) onRegister(c *gin.Context) {
	var form types.User
	var query LoginPageQuery
	if err := c.MustBindWith(&query, binding.Query); err != nil {
		c.String(http.StatusBadRequest, err.Error())
		return
	}
	if err := c.MustBindWith(&form, binding.FormPost); err != nil {
		c.String(http.StatusUnprocessableEntity, err.Error())
		return
	}

	ctx, cancel := context.WithTimeout(context.Background(), time.Second*3)
	defer cancel()
	id, err := s.mongo.AddUser(ctx, form)
	if err != nil {
		merr := err.(mongo.WriteException)
		for _, werr := range merr.WriteErrors {
			if werr.Code == 11000 {
				c.String(http.StatusConflict, "Account already exists")
				return
			}
			c.JSON(http.StatusInternalServerError, err.Error())
			return
		}
	}
	c.String(http.StatusOK, fmt.Sprintf("Account created with ID: %s, now you can log in", id.Hex()))
}

func (s *Server) onLogout(c *gin.Context) {
	strtoken := utils.ExtractHeaderToken(c.Request)
	if strtoken == nil {
		c.JSON(http.StatusUnauthorized, "Authorization token not provided")
		return
	}
	token, err := utils.VerifyToken(*strtoken, []byte(accessKey))
	if err != nil {
		c.JSON(http.StatusUnauthorized, err.Error())
		return
	}
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*3)
	defer cancel()
	_, err = s.redis.DeleteToken(ctx, token.ID)
	if err != nil {
		c.JSON(http.StatusInternalServerError, err.Error())
		return
	}
	c.JSON(http.StatusOK, "Successfully deleted")
}
