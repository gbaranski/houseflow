package server

import (
	"fmt"
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

func (s *Server) onTokenAuthorizationCodeGrant(form *TokenQuery) (gin.H, *tokenError) {
	if !validateRedirectURI(form.RedirectURI) {
		return nil, &tokenError{
			InvalidGrant: false,
			Err:          fmt.Errorf("Invalid redirect_uri"),
		}
	}
	token, err := utils.VerifyToken(form.Code, utils.JWTAuthCodeKey)
	if err != nil {
		return nil, &tokenError{
			InvalidGrant: true,
			Err:          err,
		}
	}
	tp, err := utils.CreateTokenPair()
	if err != nil {
		return nil, &tokenError{
			InvalidGrant: false,
			Err:          err,
		}
	}
	userID, err := primitive.ObjectIDFromHex(token.Claims.Audience)
	if err != nil {
		return nil, &tokenError{
			InvalidGrant: false,
			Err:          err,
		}
	}

	err = s.db.Redis.AddTokenPair(userID, tp)
	if err != nil {
		return nil, &tokenError{
			InvalidGrant: false,
			Err:          err,
		}
	}

	at := time.Unix(tp.AccessToken.Claims.ExpiresAt, 0)
	now := time.Now()
	return gin.H{
		"token_type":    "Bearer",
		"access_token":  tp.AccessToken.Token.Raw,
		"refresh_token": tp.RefreshToken.Token.Raw,
		"expires_in":    at.Sub(now),
	}, nil

}

func (s *Server) onTokenAccessTokenGrant(form *TokenQuery) (gin.H, *tokenError) {
	rt, err := utils.VerifyToken(form.RefreshToken, utils.JWTRefreshKey)
	if err != nil {
		return nil, &tokenError{
			InvalidGrant: true,
			Err:          err,
		}
	}
	_, err = s.db.Redis.FetchToken(rt.Claims)
	if err != nil {
		return nil, &tokenError{
			InvalidGrant: true,
			Err:          err,
		}
	}

	at, err := utils.CreateAccessToken()
	if err != nil {
		return nil, &tokenError{
			InvalidGrant: false,
			Err:          err,
		}
	}
	atexp := time.Unix(at.Claims.ExpiresAt, 0)
	now := time.Now()
	return gin.H{
		"token_type":   "Bearer",
		"access_token": at.Token.Raw,
		"expires_in":   atexp.Sub(now),
	}, nil
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
		json, err := s.onTokenAuthorizationCodeGrant(&form)
		if err != nil {
			if err.InvalidGrant {
				c.JSON(http.StatusBadRequest, gin.H{
					"error":   "invalid_grant",
					"message": err.Err.Error(),
				})
			} else {
				c.JSON(http.StatusInternalServerError, gin.H{})
			}
			fmt.Println("Error occured when requesting token", err.Err.Error())
			return
		}
		c.JSON(http.StatusOK, json)
	} else if form.GrantType == "refresh_token" {
		json, err := s.onTokenAccessTokenGrant(&form)
		if err != nil {
			if err.InvalidGrant {
				c.JSON(http.StatusBadRequest, gin.H{
					"error":   "invalid_grant",
					"message": err.Err.Error(),
				})
			} else {
				c.JSON(http.StatusInternalServerError, gin.H{})
			}
			fmt.Println("Error occured when requesting token", err.Err.Error())
			return
		}
		c.JSON(http.StatusOK, json)
	} else {
		c.JSON(http.StatusBadRequest, gin.H{
			"error":   "invalid_grant",
			"message": "Invalid GrantType",
		})
	}
}
