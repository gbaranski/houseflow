package types

import (
	"github.com/gbaranski/houseflow/pkg/fulfillment"
)

// Device is device from database
type Device struct {
	fulfillment.Device `bson:",inline"`
	PublicKeyBase64    string `bson:"publickey"`
}

// DeviceResponse is type of response coming out of the device
type DeviceResponse struct {
	State  map[string]interface{} `json:"state"`
	Status string                 `json:"status"`
	Error  string                 `json:"error,omitempty"`
}

// DeviceRequest is type of request incoming to the device
type DeviceRequest struct {
	State   map[string]interface{} `json:"state,omitempty"`
	Command string                 `json:"command"`
}

// DevicePermissions defines what user can and cannot
type DevicePermissions struct {
	Read    bool
	Write   bool
	Execute bool
}
