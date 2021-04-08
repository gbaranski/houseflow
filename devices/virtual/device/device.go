package device

import (
	"crypto/ed25519"
	"encoding/base64"

	"github.com/gbaranski/houseflow/devices/virtual/config"
  "github.com/gbaranski/houseflow/lighthouse/tcp_client"
  "github.com/gbaranski/houseflow/lighthouse/packets"
	"github.com/sirupsen/logrus"
)

// CreateDevice creates a device, and initializes MQTT connection
func CreateDevice(config *config.Config) Device {
	return Device{
		config: config,
		state: State{
			Online: true,
			On:     false,
		},
	}
}

// Takes public key and signes it using private key, it is password for MQTT
func (d *Device) generateMQTTPassword() string {
	sig := ed25519.Sign(d.config.PrivateKey, d.config.PublicKey)
	sigenc := base64.StdEncoding.EncodeToString(sig)

	return sigenc
}

// StartTCP Starts TCP client
func (d *Device) StartTCP() (err error) {
  cfg := tcp_client.Config{
    Host: "localhost",
    Port: 3030,
    ExecuteHandler: d.onExecute,
  }
  d.client, err = tcp_client.Connect(cfg)
  if err != nil {
    return err
  }

	return nil
}

func (d *Device) onExecute(p packets.ExecutePayload) (packets.ExecuteResponsePayload) {
	logrus.Info("Received EXECUTE")
  d.state.On = p.Params["On"].(bool) // Update state

  return packets.ExecuteResponsePayload {
    ID: p.ID,
    State: map[string]interface{}{
      "on": d.state.On,
      "online": d.state.Online,
    },
    Status: packets.ExecuteResponseStatusSuccess,
  }
}
