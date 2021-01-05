package database

import (
	"context"
)

// Database struct
type Database struct {
	Mongo *Mongo
}

// CreateDatabase creates and connect to DB
//
// Returns Database struct
func CreateDatabase(ctx context.Context) (*Database, error) {
	mongo, err := createMongo(ctx)
	if err != nil {
		return nil, err
	}
	return &Database{
		Mongo: mongo,
	}, nil
}
