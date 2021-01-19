package postgres

import (
	"context"

	log "github.com/sirupsen/logrus"

	"github.com/jackc/pgconn"
	"github.com/jackc/pgx/v4"
	"github.com/jackc/pgx/v4/log/logrusadapter"
	"github.com/jackc/pgx/v4/pgxpool"
)

// Options defines options for postgres, fields supports https://github.com/caarlos0/env tags
type Options struct {
	// Password for Postgres
	Password string `env:"POSTGRES_PASSWORD,required"`

	// Name of the database,
	DatabaseName string `env:"DATABASE_NAME" envDefault:"houseflow"`
}

// Postgres ...
type Postgres struct {
	conn *pgxpool.Pool
}

// New connect to pgxpool and returns Postgres stuct
func New(ctx context.Context, opts Options) (Postgres, error) {
	conn, err := pgxpool.ConnectConfig(ctx, &pgxpool.Config{
		ConnConfig: &pgx.ConnConfig{
			Config: pgconn.Config{
				Host:     "postgres",
				Port:     5432,
				Database: opts.DatabaseName,
				Password: opts.Password,
			},
			Logger:   logrusadapter.NewLogger(log.StandardLogger()),
			LogLevel: pgx.LogLevelInfo,
		},
	})
	if err != nil {
		return Postgres{}, err
	}

	return Postgres{
		conn: conn,
	}, nil
}
