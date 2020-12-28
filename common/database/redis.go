package database

import (
	"fmt"
	"time"

	"github.com/gbaranski/houseflow/common/token"
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

// AddTokenPair adds tokenpair to redis
func (r *Redis) AddTokenPair(userID primitive.ObjectID, tp *token.TokenPair) error {
	at := time.Unix(tp.AccessToken.Claims.ExpiresAt, 0)
	now := time.Now()

	ctx, cancel := context.WithTimeout(context.Background(), time.Second*5)
	defer cancel()

	err := r.client.Set(ctx, tp.AccessToken.Claims.Id, userID.Hex(), at.Sub(now)).Err()
	if err != nil {
		return err
	}
	err = r.client.Set(ctx, tp.RefreshToken.Claims.Id, userID.Hex(), 0).Err()
	if err != nil {
		return err
	}
	return nil
}

// DeleteToken removes token from redis
func (r *Redis) DeleteToken(tokenID string) (int64, error) {
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*5)
	defer cancel()
	deleted, err := r.client.Del(ctx, tokenID).Result()
	if err != nil {
		return 0, err
	}
	if deleted < 1 {
		return deleted, fmt.Errorf("couldn't find matching token")
	}
	return deleted, nil
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
