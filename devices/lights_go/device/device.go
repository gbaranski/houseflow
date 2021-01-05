package device

import (
	"crypto/ed25519"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"log"
	"os"
	"strings"
	"time"

	mqtt "github.com/eclipse/paho.mqtt.golang"
	"github.com/gbaranski/houseflow/devices/lights_go/config"
	"github.com/gbaranski/houseflow/devices/lights_go/utils"
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
	return string(ed25519.Sign(d.config.PrivateKey, d.config.PublicKey))
}

// StartMQTT Starts MQTT client, doesn't block, and returns MQTT Client
func (d *Device) StartMQTT() (*mqtt.Client, error) {
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

	c := mqtt.NewClient(opts)
	if token := c.Connect(); token.Wait() && token.Error() != nil {
		return nil, token.Error()
	}

	if token := c.Subscribe(d.config.CommandTopic.Request, 0, d.onCommand); token.Wait() && token.Error() != nil {
		return nil, token.Error()
	}
	return &c, nil
}

// OnCommand handles all commands
func (d *Device) onCommand(c mqtt.Client, m mqtt.Message) {
	payload, ssig, err := utils.ParsePayload(m.Payload())
	if err != nil {
		fmt.Println("Failed parsing payload: ", err.Error())
		return
	}
	ssigvalid := ed25519.Verify(ed25519.PublicKey(d.config.ServerPublicKey), []byte(payload), ssig)
	if !ssigvalid {
		fmt.Println("Server sent message with invalid signature")
		return
	}
	var req Request
	err = json.Unmarshal([]byte(payload), &req)
	if err != nil {
		fmt.Println("Failed unmarshalling request ", err.Error())
		return
	}
	fmt.Printf("Request: %+v\n", req)

	var res Response

	switch req.Command {
	case "action.devices.commands.OnOff":
		d.state.On = req.State.On
		res = Response{
			CorrelationData: req.CorrelationData,
			State:           d.state,
			Status:          "SUCCESS",
		}
	default:
		res = Response{
			CorrelationData: req.CorrelationData,
			State:           d.state,
			Status:          "ERROR",
			Error:           "functionNotSupported",
		}
	}
	resjson, err := json.Marshal(res)
	if err != nil {
		panic(err)
	}

	dsig := ed25519.Sign(d.config.PrivateKey, resjson)
	encdsig := base64.StdEncoding.EncodeToString(dsig)

	resp := strings.Join([]string{encdsig, string(resjson)}, ".")

	token := c.Publish(d.config.CommandTopic.Response, 0, false, resp)
	token.Wait()
	if token.Error() != nil {
		panic(token.Error())
	}
	fmt.Println("Successfully sent message")
}
