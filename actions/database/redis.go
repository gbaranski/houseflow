package database

import (
	"fmt"
	"time"

	"github.com/gbaranski/houseflow/actions/token"
	"github.com/go-redis/redis/v8"
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

// FetchToken fetches authentication token
func (r *Redis) FetchToken(claims token.TokenClaims) (*string, error) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*5)
	defer cancel()
	userID, err := r.client.Get(ctx, claims.Id).Result()
	if err != nil {
		if err == redis.Nil {
			return nil, fmt.Errorf("token not found, might be invalid")
		}
		return nil, err
	}
	return &userID, nil
}
