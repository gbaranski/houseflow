package postgres

import (
	"context"
	"fmt"

	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/gbaranski/houseflow/pkg/utils"
	"github.com/google/uuid"
	"github.com/jackc/pgx/v4"
)

// UsersSchema is SQL schema for users
const UsersSchema = `
CREATE TABLE IF NOT EXISTS users (
    id              UUID    PRIMARY KEY,
    first_name      TEXT    NOT NULL,
    last_name       TEXT    NOT NULL,
    email           TEXT    NOT NULL,
    password_hash   TEXT    NOT NULL -- bcrypted password
)
`

// UserDevicesSchema is SQL schema for users_devices, it defines whether user has r/w/x permission
const UserDevicesSchema = `
CREATE TABLE IF NOT EXISTS user_devices (
    user_id     UUID    REFERENCES users   (id),
    device_id   UUID    REFERENCES devices (id),
    read        BOOL    NOT NULL, -- determines if user can do query intent
    write       BOOL    NOT NULL, -- unused at the moment, probably cna be used to tell if user can invite someone
    execute     BOOL    NOT NULL, -- determines if user can do execute intent

    PRIMARY KEY (user_id, device_id)
)
`

func (p Postgres) getUser(ctx context.Context, field string, value string) (user types.User, err error) {
	sql := fmt.Sprintf("SELECT first_name, last_name, email, password_hash FROM users WHERE %s=$1", field)
	row := p.conn.QueryRow(ctx, sql, value)
	err = row.Scan(&user.FirstName, &user.LastName, &user.Email, &user.PasswordHash)
	return user, err
}

// GetUserByID queries user
func (p Postgres) GetUserByID(ctx context.Context, id string) (types.User, error) {
	return p.getUser(ctx, "id", id)
}

// GetUserByEmail queries user
func (p Postgres) GetUserByEmail(ctx context.Context, email string) (types.User, error) {
	return p.getUser(ctx, "email", email)
}

// GetUserDevices retreives user devices, only those which have READ=true
func (p Postgres) GetUserDevices(ctx context.Context, userID string) ([]types.Device, error) {
	const sql = `
		SELECT device_id 
		FROM user_devices
		WHERE user_id=$1
		AND read
	`
	cur, err := p.conn.Query(ctx, sql, userID)
	if err != nil {
		return nil, fmt.Errorf("fail query %s", err.Error())
	}

	var ids []string
	for cur.Next() {
		var id string
		err := cur.Scan(&id)
		if err != nil {
			return nil, fmt.Errorf("fail scan %s", err.Error())
		}
		ids = append(ids, id)
	}

	return p.GetDevicesByIDs(ctx, ids)
}

// GetUserDevicePermissions retreives permissions to specific device from specific user
func (p Postgres) GetUserDevicePermissions(ctx context.Context, userID string, deviceID string) (perms types.DevicePermissions, err error) {
	const sql = `
	SELECT read, write, execute
	FROM user_devices
	WHERE user_id=$1
	AND device_id=$2
	`
	row := p.conn.QueryRow(ctx, sql, userID, deviceID)
	err = row.Scan(&perms.Read, &perms.Write, &perms.Execute)
	if err == pgx.ErrNoRows {
		return types.DevicePermissions{
			Read:    false,
			Write:   false,
			Execute: false,
		}, nil
	}

	if err != nil {
		return types.DevicePermissions{}, fmt.Errorf("fail query %s", err.Error())
	}

	return perms, nil
}

// AddUser inserts new user to database
func (p Postgres) AddUser(ctx context.Context, user types.User) (id string, err error) {
	const sql = `
	INSERT INTO users
	VALUES ($1, $2, $3, $4, $5)
	`
	userID, err := uuid.NewRandom()
	if err != nil {
		return "", fmt.Errorf("fail gen uuid %s", err.Error())
	}
	passwordHash, err := utils.HashPassword([]byte(user.Password))
	if err != nil {
		return "", fmt.Errorf("fail hash password %s", err.Error())
	}
	_, err = p.conn.Exec(ctx, sql, userID.String(), user.FirstName, user.LastName, user.Email, passwordHash)
	return userID.String(), nil
}
