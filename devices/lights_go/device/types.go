package device

import (
	mqtt "github.com/eclipse/paho.mqtt.golang"
	"github.com/gbaranski/houseflow/devices/lights_go/config"
)

// State is type of current state of device
type State struct {
	Online bool `json:"online"`
	On     bool `json:"on"`
}

// Response is type of response coming out of the device
type Response struct {
	CorrelationData string `json:"correlationData"`
	State           State  `json:"state"`
	Status          string `json:"status"`
	Error           string `json:"error,omitempty"`
}

// Request is type of request incoming to the device
type Request struct {
	CorrelationData string `json:"correlationData"`
	State           State  `json:"state"`
}

// Device ...
type Device struct {
	config     *config.Config
	state      State
	MQTTClient mqtt.Client
}
