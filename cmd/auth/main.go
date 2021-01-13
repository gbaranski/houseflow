package main

import (
	"context"
	"net/http"
	"time"

	"github.com/gbaranski/houseflow/internal/auth"
	"github.com/gbaranski/houseflow/pkg/database"
	"github.com/gbaranski/houseflow/pkg/utils"
)

var (
	mongoUsername = utils.MustGetEnv("MONGO_INITDB_ROOT_USERNAME")
	mongoPassword = utils.MustGetEnv("MONGO_INITDB_ROOT_PASSWORD")

	projectID    = utils.MustGetEnv("ACTIONS_PROJECT_ID")
	clientID     = utils.MustGetEnv("OAUTH_CLIENT_ID")
	clientSecret = utils.MustGetEnv("OAUTH_CLIENT_SECRET")

	accessKey            = utils.MustGetEnv("ACCESS_KEY")
	authorizationCodeKey = utils.MustGetEnv("AUTHORIZATION_CODE_KEY")
	refreshKey           = utils.MustGetEnv("REFRESH_KEY")
)

type mergedDatabases struct {
	database.Mongo
	database.Redis
}

func main() {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	mongo, err := database.NewMongo(ctx, database.MongoOptions{
		Username:     mongoUsername,
		Password:     mongoPassword,
		DatabaseName: "houseflowDB",
	})
	if err != nil {
		panic(err)
	}

	redis, err := database.NewRedis(ctx, database.RedisOptions{})
	if err != nil {
		panic(err)
	}

	s := auth.NewAuth(mergedDatabases{
		Mongo: mongo,
		Redis: redis,
	}, auth.Options{
		ProjectID:            projectID,
		ClientID:             clientID,
		ClientSecret:         clientSecret,
		AccessKey:            accessKey,
		AuthorizationCodeKey: authorizationCodeKey,
		RefreshKey:           refreshKey,
	})
	http.ListenAndServe(":80", s.Router)
	if err != nil {
		panic(err)
	}
}
