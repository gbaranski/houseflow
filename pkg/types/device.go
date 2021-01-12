package types

import (
	"github.com/gbaranski/houseflow/pkg/fulfillment"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

// Device is device from database
type Device struct {
	fulfillment.Device `bson:",inline"`
	ID                 primitive.ObjectID     `bson:"_id,omitempty" binding:"-"`
	PublicKey          string                 `bson:"publickey"`
	State              map[string]interface{} `bson:"state"`
}

// DeviceResponse is type of response coming out of the device
type DeviceResponse struct {
	CorrelationData string                 `json:"correlationData"`
	State           map[string]interface{} `json:"state"`
	Status          string                 `json:"status"`
	Error           string                 `json:"error,omitempty"`
}

// DeviceRequest is type of request incoming to the device
type DeviceRequest struct {
	CorrelationData string                 `json:"correlationData"`
	State           map[string]interface{} `json:"state"`
	Command         string                 `json:"command"`
}
