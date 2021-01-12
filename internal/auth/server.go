package auth

import (
	"context"
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/gin-gonic/gin/binding"
	"go.mongodb.org/mongo-driver/bson/primitive"

	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/gbaranski/houseflow/pkg/utils"
)

// Mongo is interface for mongoDB for auth server
type Mongo interface {
	AddUser(ctx context.Context, user types.User) (primitive.ObjectID, error)
	GetUserByEmail(ctx context.Context, email string) (types.User, error)
}

// Redis is interface for Redis for auth server
type Redis interface {
	AddToken(ctx context.Context, userID primitive.ObjectID, token utils.Token) error
	FetchToken(ctx context.Context, token utils.Token) (string, error)
	DeleteToken(ctx context.Context, tokenID string) (int64, error)
}

// Auth hold root server state
type Auth struct {
	mongo  Mongo
	redis  Redis
	Router *gin.Engine
	opts   Options
}

// NewAuth creates server, it won't run till Auth.Router.Start
func NewAuth(mongo Mongo, redis Redis, opts Options) Auth {
	a := Auth{
		mongo:  mongo,
		redis:  redis,
		Router: gin.Default(),
		opts:   opts,
	}
	a.Router.LoadHTMLGlob("../../web/template/*")

	a.Router.GET("/auth", a.onLoginPage)

	a.Router.POST("/login", a.onLogin)
	a.Router.POST("/register", a.onRegister)
	a.Router.POST("/logout", a.onLogout)
	a.Router.POST("/token", a.onToken)

	return a
}

func (a *Auth) onLoginPage(c *gin.Context) {
	var query LoginPageQuery
	if err := c.MustBindWith(&query, binding.Query); err != nil {
		c.String(http.StatusBadRequest, err.Error())
		return
	}
	if query.ClientID != a.opts.ClientID {
		c.String(http.StatusBadRequest, "ClientID is invalid")
		return
	}
	if !a.validateRedirectURI(query.RedirectURI) {
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
