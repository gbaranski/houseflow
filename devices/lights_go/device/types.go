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

// Device ...
type Device struct {
	config *config.Config
	client mqtt.Client
	state  State
}
