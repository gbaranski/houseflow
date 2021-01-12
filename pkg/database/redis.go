package database

import (
	"fmt"
	"time"

	"github.com/gbaranski/houseflow/pkg/utils"
	"github.com/go-redis/redis/v8"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"golang.org/x/net/context"
)

// RedisOptions defines options for redis
type RedisOptions struct {
	// Username for redis
	Username string

	// Password for redis
	Password string
}

// Parse parses the options and set the defaults
func (opts *RedisOptions) Parse() {
}

// Redis contains db and client
type Redis struct {
	client *redis.Client
	opts   RedisOptions
}

// NewRedis creates Redis, connects to redisdb with given options
func NewRedis(ctx context.Context, opts RedisOptions) (Redis, error) {
	opts.Parse()
	client := redis.NewClient(&redis.Options{
		Addr:     "redis:6379",
		Username: opts.Username,
		Password: opts.Password,
		DB:       0,
	})
	_, err := client.Ping(ctx).Result()
	if err != nil {
		return Redis{}, err
	}
	return Redis{client: client, opts: opts}, nil
}

// AddToken adds token to Redis DB
func (r Redis) AddToken(ctx context.Context, userID primitive.ObjectID, token utils.Token) error {
	exp := time.Unix(token.ExpiresAt, 0)

	return r.client.Set(ctx, token.ID, userID.Hex(), time.Since(exp)).Err()
}

// DeleteToken removes token from redi
func (r Redis) DeleteToken(ctx context.Context, tokenID string) (int64, error) {
	deleted, err := r.client.Del(ctx, tokenID).Result()
	if err != nil {
		return 0, err
	}
	if deleted < 1 {
		return deleted, fmt.Errorf("couldn't find matching token")
	}
	return deleted, nil
}

// FetchToken fetches token and returns correspoding UserID
func (r Redis) FetchToken(ctx context.Context, token utils.Token) (string, error) {
	userID, err := r.client.Get(ctx, token.ID).Result()
	if err != nil {
		if err == redis.Nil {
			return "", fmt.Errorf("token not found, might be invalid")
		}
		return "", err
	}
	return userID, nil
}
