package server

import (
	"fmt"
	"math"
	"net/http"
	"os"
	"time"

	"github.com/gbaranski/houseflow/auth/utils"
	"github.com/gin-gonic/gin"
	"github.com/gin-gonic/gin/binding"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

type tokenError struct {
	InvalidGrant bool
	Err          error
}

func (s *Server) onTokenAuthorizationCodeGrant(c *gin.Context, form TokenQuery) {
	if !validateRedirectURI(form.RedirectURI) {
		c.JSON(http.StatusBadRequest, gin.H{
			"error":             "bad_request",
			"error_description": "invalid_redirect_uri",
		})
		return
	}
	token, err := utils.VerifyToken(form.Code, utils.JWTAuthCodeKey)
	if err != nil {
		c.JSON(http.StatusUnauthorized, gin.H{
			"error":             "invalid_grant",
			"error_description": err.Error(),
		})
		return
	}

	userID, err := primitive.ObjectIDFromHex(token.Claims.Audience)
	if err != nil {
		c.JSON(http.StatusUnauthorized, gin.H{
			"error":             "invalid_grant",
			"error_description": err.Error(),
		})
		return
	}

	rt, err := utils.CreateRefreshToken(userID)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"error":             "rt_create_fail",
			"error_description": err.Error(),
		})
		return
	}
	at, err := utils.CreateAccessToken(userID)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"error":             "at_create_fail",
			"error_description": err.Error(),
		})
		return
	}

	err = s.db.Redis.AddToken(userID, rt)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"error":             "Failed adding token pair",
			"error_description": err.Error(),
		})
		return
	}

	atexp := time.Unix(at.Claims.ExpiresAt, 0)
	now := time.Now()
	c.JSON(http.StatusOK, gin.H{
		"token_type":    "Bearer",
		"access_token":  at.Token.Raw,
		"refresh_token": rt.Token.Raw,
		"expires_in":    math.Round(atexp.Sub(now).Seconds()),
	})
}

func (s *Server) onTokenAccessTokenGrant(c *gin.Context, form TokenQuery) {
	fmt.Println("Attempt to get access token")
	rt, err := utils.VerifyToken(form.RefreshToken, utils.JWTRefreshKey)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{
			"error":             "invalid_grant",
			"error_description": err.Error(),
		})
		return
	}
	userID, err := s.db.Redis.FetchToken(rt.Claims)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{
			"error":             "invalid_grant",
			"error_description": err.Error(),
		})
		return
	}

	userIDObject, err := primitive.ObjectIDFromHex(*userID)
	if err != nil {
		fmt.Println("Unable to parse userID to objectID")
		c.JSON(http.StatusBadRequest, gin.H{
			"error":             "invalid_grant",
			"error_description": err.Error(),
		})
		return
	}
	at, err := utils.CreateAccessToken(userIDObject)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"error":             "Failed creating access token",
			"error_description": err.Error(),
		})
		return
	}
	atexp := time.Unix(at.Claims.ExpiresAt, 0)
	now := time.Now()
	c.JSON(http.StatusOK, gin.H{
		"token_type":   "Bearer",
		"access_token": at.Token.Raw,
		"expires_in":   math.Round(atexp.Sub(now).Seconds()),
	})
}

func (s *Server) onToken(c *gin.Context) {
	var form TokenQuery
	if err := c.MustBindWith(&form, binding.FormPost); err != nil {
		c.String(http.StatusBadRequest, err.Error())
		return
	}
	if form.ClientID != os.Getenv(ClientIDEnv) || form.ClientSecret != os.Getenv(ClientSecretEnv) {
		c.JSON(http.StatusForbidden, gin.H{
			"error":   "invalid_grant",
			"message": "Invalid clientID or clientSecret",
		})
		return
	}
	if form.GrantType == "authorization_code" {
		s.onTokenAuthorizationCodeGrant(c, form)
	} else if form.GrantType == "refresh_token" {
		s.onTokenAccessTokenGrant(c, form)
	} else {
		c.JSON(http.StatusBadRequest, gin.H{
			"error":   "invalid_grant",
			"message": "Invalid GrantType",
		})
	}
}
