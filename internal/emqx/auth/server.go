package auth

import (
	"context"
	"crypto/ed25519"

	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/go-chi/chi"
	"github.com/go-chi/chi/middleware"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

// Options of auth
type Options struct {
	ServerPublicKey ed25519.PublicKey
}

// Database is interface for database specifiacllyl for atuh
type Database interface {
	GetDeviceByID(ctx context.Context, ID primitive.ObjectID) (types.Device, error)
}

// Auth holds server state
type Auth struct {
	db     Database
	Router *chi.Mux
	opts   Options
}

// New returns auth
func New(db Database, opts Options) Auth {
	a := Auth{
		db:     db,
		Router: chi.NewRouter(),
		opts:   opts,
	}

	a.Router.Use(middleware.Logger)
	a.Router.Post("/user", a.onConnect)
	a.Router.Post("/acl", a.onACL)
	return a
}
