package devmgmt

import (
	"context"
	"crypto/ed25519"
	"encoding/base64"
	"encoding/json"
	"errors"
	"fmt"
	"log"
	"os"
	"strings"

	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/gbaranski/houseflow/pkg/utils"

	paho "github.com/eclipse/paho.mqtt.golang"
)

// Options of the Devmgmt
type Options struct {
	// ClientID, required
	ClientID string

	// Default: "tcp://emqx:1883/mqtt"
	BrokerURL string

	// ServerPublicKey is servers public key
	//
	// *Required*
	ServerPublicKey ed25519.PublicKey

	// ServerPrivateKey is servers private key
	//
	// *Required*
	ServerPrivateKey ed25519.PrivateKey
}

// Parse parses options to the defaults
func (opts *Options) Parse() {
	if opts.BrokerURL == "" {
		opts.BrokerURL = "tcp://emqx:1883/mqtt"
	}
	if opts.ServerPublicKey == nil {
		panic("ServerPublicKey option is required")
	}
	if opts.ServerPrivateKey == nil {
		panic("ServerPrivateKey option is required")
	}
}

// Devmgmt is some abstraction layer over paho mqtt
type Devmgmt struct {
	Client paho.Client
	opts   Options
}

// New is constructor for MQTT, connects to broker
func New(opts Options) (Devmgmt, error) {
	paho.ERROR = log.New(os.Stdout, "[ERROR] ", 0)
	paho.CRITICAL = log.New(os.Stdout, "[CRIT] ", 0)
	paho.WARN = log.New(os.Stdout, "[WARN]  ", 0)
	// mqtt.DEBUG = log.New(os.Stdout, "[DEBUG] ", 0)

	username := base64.StdEncoding.EncodeToString(opts.ServerPublicKey)
	password := base64.StdEncoding.EncodeToString(ed25519.Sign(opts.ServerPrivateKey, opts.ServerPublicKey))

	copts := paho.
		NewClientOptions().
		AddBroker(opts.BrokerURL).
		SetClientID(opts.ClientID).
		SetUsername(username).
		SetPassword(password).
		SetAutoReconnect(true).
		SetConnectRetry(true)

	c := paho.NewClient(copts)
	if token := c.Connect(); token.Wait() && token.Error() != nil {
		return Devmgmt{}, token.Error()
	}
	return Devmgmt{
		Client: c,
		opts:   opts,
	}, nil
}

// ErrDeviceTimeout indicates that device had timeout
var ErrDeviceTimeout = errors.New("device timeout")

// ErrInvalidSignature indicates that device sent back invalid singature of payload
var ErrInvalidSignature = errors.New("invalid signature")

// FetchDeviceState sends rqeuest to device with question about his device state
func (d Devmgmt) FetchDeviceState(ctx context.Context, deviceID string) (types.DeviceResponse, error) {
	panic(fmt.Errorf("not implemented"))
}

// SendCommand sends request and waits for response and returns it
func (d Devmgmt) SendCommand(ctx context.Context, device types.Device, command string, params map[string]interface{}) (types.DeviceResponse, error) {
	reqTopic := fmt.Sprintf("%s/command/request", device.ID)
	resTopic := fmt.Sprintf("%s/command/response", device.ID)
	msgc := make(chan paho.Message)

	if token := d.Client.Subscribe(resTopic, 0, func(c paho.Client, m paho.Message) {
		msgc <- m
	}); token.Wait() && token.Error() != nil {
		return types.DeviceResponse{}, token.Error()
	}

	defer func() {
		d.Client.Unsubscribe(resTopic)
	}()

	req := types.DeviceRequest{
		CorrelationData: utils.GenerateRandomString(16),
		State:           params,
		Command:         command,
	}
	reqjson, err := json.Marshal(req)
	if err != nil {
		return types.DeviceResponse{}, err
	}

	ssig := ed25519.Sign(d.opts.ServerPrivateKey, reqjson)
	encssig := base64.StdEncoding.EncodeToString(ssig)

	reqp := strings.Join([]string{encssig, string(reqjson)}, ".")

	if token := d.Client.Publish(reqTopic, 0, false, reqp); token.Wait() && token.Error() != nil {
		return types.DeviceResponse{}, token.Error()
	}

readMessages:
	for {
		select {
		case <-ctx.Done():
			return types.DeviceResponse{}, ErrDeviceTimeout

		case msg, ok := <-msgc:
			fmt.Println("Received some message")
			if !ok {
				fmt.Println("Failed waiting for msg for unknown reason")
				continue readMessages
			}

			resp := msg.Payload()
			resjson, dsig, err := utils.ParseSignedPayload(resp)
			if err != nil {
				fmt.Println("Failed parsing payload to json and sig: ", err.Error())
				continue readMessages
			}
			var res types.DeviceResponse
			err = json.Unmarshal([]byte(resjson), &res)
			if err != nil {
				fmt.Println("Failed unmarshalling json: ", err.Error())
				continue readMessages
			}
			if res.CorrelationData != req.CorrelationData {
				fmt.Println("Correlation data doesn't match, skipping")
				continue readMessages
			}
			// TODO: make database store raw bin
			dpkey, err := base64.StdEncoding.DecodeString(device.PublicKey)
			if err != nil {
				fmt.Println("fail parsing device public key: ", err.Error())
				return types.DeviceResponse{}, fmt.Errorf("fail parsing device public key %s", err.Error())
			}
			valid := ed25519.Verify(ed25519.PublicKey(dpkey), []byte(resjson), dsig)
			if !valid {
				return types.DeviceResponse{}, ErrInvalidSignature
			}
			return res, nil
		}
	}
}
