package database

import (
	"time"

	"github.com/gbaranski/houseflow/auth/utils"
	"github.com/go-redis/redis/v8"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"golang.org/x/net/context"
)

// Redis contains db and client
type Redis struct {
	client *redis.Client
}

func createRedis() (*Redis, error) {
	rdb := redis.NewClient(&redis.Options{
		Addr:     "redis:6379",
		Password: "",
		DB:       0,
	})
	pctx, cancel := context.WithTimeout(context.Background(), time.Second*5)
	defer cancel()
	_, err := rdb.Ping(pctx).Result()
	if err != nil {
		return nil, err
	}
	return &Redis{client: rdb}, nil
}

// CreateAuth creates auth and adds token to redis
func (r *Redis) CreateAuth(userID primitive.ObjectID, td *utils.Tokens) error {
	at := time.Unix(td.AccessToken.Claims.ExpiresAt, 0)
	rt := time.Unix(td.RefreshToken.Claims.ExpiresAt, 0)
	now := time.Now()
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*5)
	defer cancel()
	err := r.client.Set(ctx, td.AccessToken.Claims.Id, userID.Hex(), at.Sub(now)).Err()
	if err != nil {
		return err
	}
	err = r.client.Set(ctx, td.RefreshToken.Claims.Id, userID.Hex(), rt.Sub(now)).Err()
	if err != nil {
		return err
	}
	return nil

}

// FetchAuth fetches authentication token
func (r *Redis) FetchAuth(claims *utils.TokenClaims) (*string, error) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*5)
	defer cancel()
	userID, err := r.client.Get(ctx, claims.Id).Result()
	if err != nil {
		return nil, err
	}
	return &userID, nil
}
