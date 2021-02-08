package postgres

import (
	"context"
	"fmt"
	"strings"

	"github.com/gbaranski/houseflow/pkg/fulfillment"
	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/google/uuid"
)

// DevicesSchema is SQL schema for devices
const DevicesSchema = `
CREATE TABLE IF NOT EXISTS devices (
    id                  UUID,
    publickey           CHAR(44)    NOT NULL,
    type                TEXT        NOT NULL,
    name   				TEXT        NOT NULL,
    will_report_state   BOOL        NOT NULL,
    room_hint           TEXT        NOT NULL,

    manufacturer        TEXT        NOT NULL,
    model               TEXT        NOT NULL,
    hw_version          TEXT        NOT NULL,
	sw_version          TEXT        NOT NULL,
	
	PRIMARY KEY (id)
)
`

// DeviceTraitsSchema is SQL schema for device_traits
const DeviceTraitsSchema = `
CREATE TABLE IF NOT EXISTS device_traits (
    id          UUID,
    device_id   UUID REFERENCES devices (id) ON DELETE CASCADE NOT NULL,
	  name        TEXT NOT NULL,

	PRIMARY KEY (id)
)
`

// GetDeviceTraits retrieves all traits for specific device
func (p Postgres) GetDeviceTraits(ctx context.Context, id string) ([]string, error) {
	const sql = "SELECT name FROM device_traits WHERE device_id=$1"
	rows, err := p.conn.Query(ctx, sql, id)
	if err != nil {
		return nil, err
	}
	traits := make([]string, 0)

	for rows.Next() {
		var trait string
		if err := rows.Scan(&trait); err != nil {
			return nil, err
		}
		traits = append(traits, trait)
	}
	return traits, nil
}

// GetDeviceByID retreives device with given ID
func (p Postgres) GetDeviceByID(ctx context.Context, id string) (*types.Device, error) {
	const sql = "SELECT publickey, type, name, will_report_state, room_hint, manufacturer, model, hw_version, sw_version FROM devices WHERE id=$1"
	row := p.conn.QueryRow(ctx, sql, id)
	device := types.Device{
		Device: fulfillment.Device{
			ID: id,
		},
	}
	err := row.Scan(
		&device.PublicKeyBase64,
		&device.Type,
		&device.Name.Name,
		&device.WillReportState,
		&device.RoomHint,
		&device.DeviceInfo.Manufacturer,
		&device.DeviceInfo.Model,
		&device.DeviceInfo.HwVersion,
		&device.DeviceInfo.SwVersion,
	)
	if err != nil {
		return nil, err
	}

	device.Traits, err = p.GetDeviceTraits(ctx, id)
	if err != nil {
		return nil, fmt.Errorf("fail retrive traits %s", err.Error())
	}
	return &device, err
}

// GetDevicesByIDs retreives device with given ID slice
func (p Postgres) GetDevicesByIDs(ctx context.Context, ids []string) (devices []types.Device, err error) {
	const sql = "SELECT id, publickey, type, name, will_report_state, room_hint, manufacturer, model, hw_version, sw_version FROM devices WHERE id IN (%s)"

	if len(ids) < 1 {
		return make([]types.Device, 0), nil
	}

	for i, id := range ids {
		// verify ids to prevent SQL injection
		if _, err := uuid.Parse(id); err != nil {
			return nil, fmt.Errorf("invalid uuid %s", err.Error())
		}
		// add surrounding ''
		ids[i] = fmt.Sprintf("'%s'", id)
	}

	rows, err := p.conn.Query(ctx, fmt.Sprintf(sql, strings.Join(ids, ",")))
	if err != nil {
		return nil, err
	}
	for rows.Next() {
		device := types.Device{}
		err = rows.Scan(
			&device.ID,
			&device.PublicKeyBase64,
			&device.Type,
			&device.Name.Name,
			&device.WillReportState,
			&device.RoomHint,
			&device.DeviceInfo.Manufacturer,
			&device.DeviceInfo.Model,
			&device.DeviceInfo.HwVersion,
			&device.DeviceInfo.SwVersion,
		)
		if err != nil {
			return nil, err
		}
		device.Traits, err = p.GetDeviceTraits(ctx, device.ID)
		if err != nil {
			return nil, fmt.Errorf("fail retrive traits %s", err.Error())
		}
		devices = append(devices, device)
	}

	return devices, err
}

// AddDevice adds device to database
func (p Postgres) AddDevice(ctx context.Context, device types.Device) (string, error) {
	const sql = `
	INSERT INTO devices (id, publickey, type, name, will_report_state, room_hint, manufacturer, model, hw_version, sw_version)
	VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
	`
	deviceID := uuid.New()
	_, err := p.conn.Exec(ctx, sql,
		deviceID.String(),
		device.PublicKeyBase64,
		device.Type,
		device.Name.Name,
		device.WillReportState,
		device.RoomHint,
		device.DeviceInfo.Manufacturer,
		device.DeviceInfo.Model,
		device.DeviceInfo.HwVersion,
		device.DeviceInfo.SwVersion,
	)
	if err != nil {
		return "", err
	}

	for _, trait := range device.Traits {
		const add_trait_sql = `INSERT INTO device_traits (id, device_id, name) VALUES ($1, $2, $3)`
		_, err := p.conn.Exec(ctx, add_trait_sql, uuid.New().String(), deviceID, trait)
		if err != nil {
			return "", fmt.Errorf("fail adding trait %s", err.Error())
		}
	}

	return deviceID.String(), err
}
