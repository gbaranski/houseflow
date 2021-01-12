package auth

import (
	"context"
	"fmt"
	"net/http"
	"time"

	"github.com/gbaranski/houseflow/pkg/utils"
	"github.com/gin-gonic/gin"
	"github.com/gin-gonic/gin/binding"
	"github.com/google/uuid"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

var ()

type tokenError struct {
	InvalidGrant bool
	Err          error
}

func newRefreshToken(userID primitive.ObjectID) (token utils.Token, strtoken string, err error) {
	id, err := uuid.NewRandom()
	if err != nil {
		return
	}
	token = utils.Token{
		Audience: userID.Hex(),
		ID:       id.String(),
	}
	strtoken, err = token.Sign([]byte(refreshKey))
	return
}

func newAccessToken(userID primitive.ObjectID) (token utils.Token, strtoken string, err error) {
	id, err := uuid.NewRandom()
	if err != nil {
		return
	}
	token = utils.Token{
		Audience:  userID.Hex(),
		ID:        id.String(),
		ExpiresAt: time.Now().Add(utils.AccessTokenDuration).Unix(),
	}
	strtoken, err = token.Sign([]byte(accessKey))
	return
}

func (s *Server) onTokenAuthorizationCodeGrant(c *gin.Context, form TokenQuery) {
	if !validateRedirectURI(form.RedirectURI) {
		c.JSON(http.StatusBadRequest, gin.H{
			"error":             "bad_request",
			"error_description": "invalid_redirect_uri",
		})
		return
	}
	fmt.Println(form.Code)
	token, err := utils.VerifyToken(form.Code, []byte(authorizationCodeKey))
	if err != nil {
		c.JSON(http.StatusUnauthorized, gin.H{
			"error":             "invalid_code",
			"error_description": err.Error(),
		})
		return
	}

	userID, err := primitive.ObjectIDFromHex(token.Audience)
	if err != nil {
		c.JSON(http.StatusUnauthorized, gin.H{
			"error":             "invalid_token_audience",
			"error_description": err.Error(),
		})
		return
	}

	rt, rtstr, err := newRefreshToken(userID)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"error":             "rt_create_fail",
			"error_description": err.Error(),
		})
		return
	}

	_, atstr, err := newAccessToken(userID)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"error":             "at_create_fail",
			"error_description": err.Error(),
		})
		return
	}

	ctx, cancel := context.WithTimeout(context.Background(), time.Second*3)
	defer cancel()
	s.redis.AddToken(ctx, userID, rt)

	c.JSON(http.StatusOK, gin.H{
		"token_type":    "Bearer",
		"access_token":  atstr,
		"refresh_token": rtstr,
		"expires_in":    int(utils.AccessTokenDuration.Seconds()),
	})
}

func (s *Server) onTokenAccessTokenGrant(c *gin.Context, form TokenQuery) {
	rt, err := utils.VerifyToken(form.RefreshToken, []byte(refreshKey))
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{
			"error":             "invalid_grant",
			"error_description": err.Error(),
		})
		return
	}

	ctx, cancel := context.WithTimeout(context.Background(), time.Second*3)
	defer cancel()
	userID, err := s.redis.FetchToken(ctx, *rt)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{
			"error":             "invalid_grant",
			"error_description": err.Error(),
		})
		return
	}

	userIDObject, err := primitive.ObjectIDFromHex(userID)
	if err != nil {
		fmt.Println("Unable to parse userID to objectID")
		c.JSON(http.StatusBadRequest, gin.H{
			"error":             "invalid_grant",
			"error_description": err.Error(),
		})
		return
	}
	_, atstr, err := newAccessToken(userIDObject)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"error":             "at_create_fail",
			"error_description": err.Error(),
		})
		return
	}
	c.JSON(http.StatusOK, gin.H{
		"token_type":   "Bearer",
		"access_token": atstr,
		"expires_in":   int(utils.AccessTokenDuration.Seconds()),
	})
}

func (s *Server) onToken(c *gin.Context) {
	var form TokenQuery
	if err := c.MustBindWith(&form, binding.FormPost); err != nil {
		c.String(http.StatusBadRequest, err.Error())
		return
	}
	if form.ClientID != clientID || form.ClientSecret != clientSecret {
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
