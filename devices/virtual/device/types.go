package device

import (
	"github.com/gbaranski/houseflow/devices/virtual/config"
	"github.com/gbaranski/houseflow/lighthouse/tcp_client"
)

// State is type of current state of device
type State struct {
	Online bool `json:"online"`
	On     bool `json:"on"`
}

// Device ...
type Device struct {
	config *config.Config
	client tcp_client.Client
	state  State
}
