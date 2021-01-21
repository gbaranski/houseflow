package postgres

import (
	"context"
	"fmt"
	"net"
	"time"

	log "github.com/sirupsen/logrus"

	"github.com/jackc/pgx/v4"
	"github.com/jackc/pgx/v4/log/logrusadapter"
	"github.com/jackc/pgx/v4/pgxpool"
)

// Options defines options for postgres, fields supports https://github.com/caarlos0/env tags
type Options struct {
	Username string `env:"POSTGRES_USER" envDefault:"postgres"`
	// Password for Postgres
	Password string `env:"POSTGRES_PASSWORD,required"`

	// Name of the database,
	DatabaseName string `env:"POSTGRES_DB" envDefault:"houseflow"`
}

// Postgres ...
type Postgres struct {
	conn *pgxpool.Pool
}

func waitPostgresReady(timeout time.Duration) error {
	ticker := time.NewTicker(time.Millisecond * 500)
waitLoop:
	for {
		select {
		case <-ticker.C:
			conn, err := net.Dial("tcp", "postgres:5432")
			if err != nil {
				fmt.Println("received err", err.Error())
				continue waitLoop
			}
			conn.Close()
			return nil
		case <-time.After(timeout):
			return fmt.Errorf("timeout")
		}
	}
}

// New connect to pgxpool and returns Postgres stuct
func New(ctx context.Context, opts Options) (Postgres, error) {
	waitPostgresReady(time.Second * 5)
	config, err := pgxpool.ParseConfig("postgresql://postgres:5432")
	if err != nil {
		return Postgres{}, err
	}
	config.ConnConfig.Database = opts.DatabaseName
	config.ConnConfig.Password = opts.Password
	config.ConnConfig.User = opts.Username
	config.ConnConfig.Logger = logrusadapter.NewLogger(log.StandardLogger())
	config.ConnConfig.LogLevel = pgx.LogLevelInfo

	conn, err := pgxpool.ConnectConfig(ctx, config)
	if err != nil {
		return Postgres{}, err
	}

	fmt.Println("Connected to PostgreSQL")
	return Postgres{
		conn: conn,
	}, nil
}
