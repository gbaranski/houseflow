package auth

import (
	"context"
	"fmt"
	"net/http"
	"net/url"
	"time"

	"github.com/gbaranski/houseflow/pkg/utils"
	"github.com/gin-gonic/gin"
	"github.com/gin-gonic/gin/binding"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

func (a *Auth) onTokenAuthorizationCodeGrant(c *gin.Context, form TokenQuery) {
	fmt.Println("Requesting authorization code grant")
	if !a.validateRedirectURI(form.RedirectURI) {
		c.JSON(http.StatusBadRequest, gin.H{
			"error":             "bad_request",
			"error_description": "invalid_redirect_uri",
		})
		return
	}
	fmt.Println(form.Code)
	authorizationCode, err := url.QueryUnescape(form.Code)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{
			"error":             "invalid_auth_code",
			"error_description": err.Error(),
		})
		return
	}
	token, err := utils.VerifyToken(authorizationCode, []byte(a.opts.AuthorizationCodeKey))
	if err != nil {
		c.JSON(http.StatusUnauthorized, gin.H{
			"error":             "invalid_auth_code",
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

	rt, rtstr, err := a.newRefreshToken(userID)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"error":             "rt_create_fail",
			"error_description": err.Error(),
		})
		return
	}

	_, atstr, err := a.newAccessToken(userID)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"error":             "at_create_fail",
			"error_description": err.Error(),
		})
		return
	}

	ctx, cancel := context.WithTimeout(context.Background(), time.Second*3)
	defer cancel()
	a.redis.AddToken(ctx, userID, rt)

	c.JSON(http.StatusOK, gin.H{
		"token_type":    "Bearer",
		"access_token":  atstr,
		"refresh_token": rtstr,
		"expires_in":    int(utils.AccessTokenDuration.Seconds()),
	})
}

func (a *Auth) onTokenAccessTokenGrant(c *gin.Context, form TokenQuery) {
	fmt.Println("Requesting access token grant")
	rt, err := utils.VerifyToken(form.RefreshToken, []byte(a.opts.RefreshKey))
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{
			"error":             "invalid_grant",
			"error_description": err.Error(),
		})
		return
	}

	ctx, cancel := context.WithTimeout(context.Background(), time.Second*3)
	defer cancel()
	userID, err := a.redis.FetchToken(ctx, *rt)
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
	_, atstr, err := a.newAccessToken(userIDObject)
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

func (a *Auth) onToken(c *gin.Context) {
	var form TokenQuery
	if err := c.MustBindWith(&form, binding.FormPost); err != nil {
		c.String(http.StatusBadRequest, err.Error())
		return
	}
	if form.ClientID != a.opts.ClientID || form.ClientSecret != a.opts.ClientSecret {
		c.JSON(http.StatusForbidden, gin.H{
			"error":   "invalid_grant",
			"message": "Invalid clientID or clientSecret",
		})
		return
	}
	if form.GrantType == "authorization_code" {
		a.onTokenAuthorizationCodeGrant(c, form)
	} else if form.GrantType == "refresh_token" {
		a.onTokenAccessTokenGrant(c, form)
	} else {
		c.JSON(http.StatusBadRequest, gin.H{
			"error":   "invalid_grant",
			"message": "Invalid GrantType",
		})
	}
}
