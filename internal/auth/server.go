package auth

import (
	"context"
	"net/http"
	"text/template"

	"github.com/go-chi/chi"
	"github.com/go-chi/chi/middleware"
	"github.com/gorilla/schema"
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
	Router *chi.Mux
	opts   Options
}

var decoder = schema.NewDecoder()
var encoder = schema.NewEncoder()

// NewAuth creates server, it won't run till Auth.Router.Start
func NewAuth(mongo Mongo, redis Redis, opts Options) Auth {
	a := Auth{
		mongo:  mongo,
		redis:  redis,
		Router: chi.NewRouter(),
		opts:   opts,
	}
	a.Router.Use(middleware.Logger)

	a.Router.Get("/auth", a.onAuthSite)

	a.Router.Post("/login", a.onLogin)
	a.Router.Post("/register", a.onRegister)
	a.Router.Post("/token", a.onToken)

	return a
}

func (a *Auth) onAuthSite(w http.ResponseWriter, r *http.Request) {
	var query LoginPageQuery

	if err := decoder.Decode(&query, r.URL.Query()); err != nil {
		http.Error(w, err.Error(), http.StatusUnprocessableEntity)
		return
	}

	if query.ClientID != a.opts.ClientID {
		http.Error(w, "ClientID is invalid", http.StatusForbidden)
		return
	}
	if !a.validateRedirectURI(query.RedirectURI) {
		http.Error(w, "redirect_uri is invalid", http.StatusBadRequest)
		return
	}
	tmpl, err := template.ParseFiles("../../web/template/auth.tmpl")
	if err != nil {
		http.Error(w, "Fail loading template", http.StatusInternalServerError)
		return
	}
	tmpl.Execute(w, map[string]string{
		"redirect_uri": query.RedirectURI,
		"state":        query.State,
	})
}
