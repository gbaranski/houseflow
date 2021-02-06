package device

import (
	"crypto/ed25519"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"log"
	"os"
	"time"

	mqtt "github.com/eclipse/paho.mqtt.golang"
	"github.com/gbaranski/houseflow/devices/lights_go/config"
	"github.com/gbaranski/houseflow/pkg/devmgmt"
	"github.com/gbaranski/houseflow/pkg/types"
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

// StartMQTT Starts MQTT client, doesn't block, and returns MQTT Client
func (d *Device) StartMQTT() error {
	mqtt.ERROR = log.New(os.Stdout, "[ERROR] ", 0)
	mqtt.CRITICAL = log.New(os.Stdout, "[CRIT] ", 0)
	mqtt.WARN = log.New(os.Stdout, "[WARN]  ", 0)
	// mqtt.DEBUG = log.New(os.Stdout, "[DEBUG] ", 0)

	// Add there some kind of password soon
	opts := mqtt.
		NewClientOptions().
		AddBroker(d.config.BrokerURL).
		SetClientID(d.config.DeviceID).
		SetUsername(base64.StdEncoding.EncodeToString(d.config.PublicKey)).
		SetPassword(d.generateMQTTPassword())

	opts.SetKeepAlive(5 * time.Second)
	opts.SetPingTimeout(1 * time.Second)

	d.client = mqtt.NewClient(opts)
	if token := d.client.Connect(); token.Wait() && token.Error() != nil {
		return token.Error()
	}

	if token := d.client.Subscribe(d.config.CommandTopic.Request, 0, d.onCommand); token.Wait() && token.Error() != nil {
		return token.Error()
	}
	if token := d.client.Subscribe(d.config.StateTopic.Request, 0, d.onFetchState); token.Wait() && token.Error() != nil {
		return token.Error()
	}
	return nil
}

func (d *Device) onFetchState(c mqtt.Client, m mqtt.Message) {
	logrus.Info("Received fetch state request")
	p := m.Payload()
	sig := p[:ed25519.SignatureSize]
	requestID := p[ed25519.SignatureSize : ed25519.SignatureSize+devmgmt.RequestIDSize]

	valid := ed25519.Verify(ed25519.PublicKey(d.config.ServerPublicKey), requestID, sig)
	if !valid {
		logrus.Error("Server sent message with invalid signature")
		return
	}
	deviceResponse := types.DeviceResponse{
		State: map[string]interface{}{
			"on":     d.state.On,
			"online": d.state.Online,
		},
		Status: "SUCCESS",
	}
	responeJSON, err := json.Marshal(deviceResponse)
	if err != nil {
		logrus.Error("fail marshall response", responeJSON)
		return
	}

	responseSignature := ed25519.Sign(d.config.PrivateKey, responeJSON)
	response := append(responseSignature, append(requestID, responeJSON...)...)

	token := d.client.Publish(d.config.StateTopic.Response, 0, false, response)
	if token.Wait(); token.Error() != nil {
		logrus.Error("Fail publishing state %s", err.Error())
	}
	logrus.WithField("json", deviceResponse).Info("Sent response")
}

// OnCommand handles all commands
func (d *Device) onCommand(c mqtt.Client, m mqtt.Message) {
	p := m.Payload()
	sig := p[:ed25519.SignatureSize]
	requestID := p[ed25519.SignatureSize : ed25519.SignatureSize+devmgmt.RequestIDSize]
	body := p[ed25519.SignatureSize+devmgmt.RequestIDSize:]

	valid := ed25519.Verify(ed25519.PublicKey(d.config.ServerPublicKey), append(requestID, body...), sig)
	if !valid {
		fmt.Println("Server sent message with invalid signature")
		logrus.Errorf("Server sent message with invalid signature")
		return
	}

	var req types.DeviceRequest
	err := json.Unmarshal([]byte(body), &req)
	if err != nil {
		logrus.Errorf("Fail marshalling request json %s", err.Error())
		return
	}
	fmt.Printf("Command request: %+v\n", req)

	var deviceResponse types.DeviceResponse

	fmt.Println("Received command:", req.Command)
	switch req.Command {
	case "action.devices.commands.OnOff":
		d.state.On = req.State["on"].(bool)
		deviceResponse = types.DeviceResponse{
			Status: "SUCCESS",
		}
	default:
		deviceResponse = types.DeviceResponse{
			Status: "ERROR",
			Error:  "functionNotSupported",
		}
	}
	deviceResponse.State = map[string]interface{}{
		"on":     d.state.On,
		"online": d.state.Online,
	}
	resjson, err := json.Marshal(deviceResponse)
	if err != nil {
		logrus.Errorf("Fail marshalling response json %s", err.Error())
		return
	}

	responeSignature := ed25519.Sign(d.config.PrivateKey, append(requestID, resjson...))
	response := append(append(responeSignature, requestID...), resjson...)

	token := c.Publish(d.config.CommandTopic.Response, 0, false, response)
	token.Wait()
	if token.Error() != nil {
		logrus.Errorf("Fail publishing response %s", token.Error().Error())
		return
	}
	logrus.WithField("json", resjson).Info("Sent response")
}
