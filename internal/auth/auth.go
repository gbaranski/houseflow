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
	"go.mongodb.org/mongo-driver/mongo"
)

func (a *Auth) onLogin(c *gin.Context) {
	var form LoginRequest
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
	dbUser, err := a.mongo.GetUserByEmail(ctx, form.Email)
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
	redirectURI, err := a.createRedirectURI(&query, dbUser.ID)
	if err != nil {
		c.String(http.StatusInternalServerError, err.Error())
		return
	}
	c.Redirect(http.StatusSeeOther, redirectURI)

}

func (a *Auth) onRegister(c *gin.Context) {
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
	id, err := a.mongo.AddUser(ctx, form)
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

func (a *Auth) onLogout(c *gin.Context) {
	strtoken := utils.ExtractHeaderToken(c.Request)
	if strtoken == nil {
		c.JSON(http.StatusUnauthorized, "Authorization token not provided")
		return
	}
	token, err := utils.VerifyToken(*strtoken, []byte(a.opts.AccessKey))
	if err != nil {
		c.JSON(http.StatusUnauthorized, err.Error())
		return
	}
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*3)
	defer cancel()
	_, err = a.redis.DeleteToken(ctx, token.ID)
	if err != nil {
		c.JSON(http.StatusInternalServerError, err.Error())
		return
	}
	c.JSON(http.StatusOK, "Successfully deleted")
}
