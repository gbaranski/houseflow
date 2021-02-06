package devmgmt

import (
	"bytes"
	"context"
	"crypto/ed25519"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"log"
	"os"

	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/gbaranski/houseflow/pkg/utils"
	"github.com/sirupsen/logrus"

	paho "github.com/eclipse/paho.mqtt.golang"
)

const (
	// RequestIDSize is size of RequestID
	RequestIDSize = 16
)

// Devmgmt is some abstraction layer over paho mqtt
type Devmgmt struct {
	Client paho.Client
	opts   Options
}

// New is constructor for MQTT, connects to broker
func New(opts Options) (Devmgmt, error) {
	opts.Parse()
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

// PublishWithResponse publishes message and waits for response and returns its body
func (devmgmt Devmgmt) PublishWithResponse(ctx context.Context, body []byte, topic ResponseTopic, pkey ed25519.PublicKey) ([]byte, error) {
	msgCh := make(chan paho.Message)

	if token := devmgmt.Client.Subscribe(topic.Response, 0, func(c paho.Client, m paho.Message) {
		msgCh <- m
	}); token.Wait() && token.Error() != nil {
		return nil, token.Error()
	}
	defer func() {
		devmgmt.Client.Unsubscribe(topic.Response)
	}()
	requestID, err := utils.NewRequestID(RequestIDSize)
	if err != nil {
		return nil, fmt.Errorf("fail gen requestID %s", err.Error())
	}

	payload := make([]byte, ed25519.SignatureSize+RequestIDSize+len(body))
	copy(payload[0:ed25519.SignatureSize], ed25519.Sign(devmgmt.opts.ServerPrivateKey, append(requestID, body...)))
	copy(payload[ed25519.SignatureSize:ed25519.SignatureSize+RequestIDSize], requestID)
	copy(payload[ed25519.SignatureSize+RequestIDSize:], body)

	if token := devmgmt.Client.Publish(topic.Request, 0, false, payload); token.Wait() && token.Error() != nil {
		return nil, token.Error()
	}

loop:
	for {
		select {
		case <-ctx.Done():
			return nil, ErrDeviceTimeout
		case msg, ok := <-msgCh:
			{
				if !ok {
					logrus.Infoln("fail on message channel")
					continue loop
				}
				responsePayload := msg.Payload()
				responseSignature := responsePayload[0:ed25519.SignatureSize]
				responseRequestID := responsePayload[ed25519.SignatureSize : ed25519.SignatureSize+RequestIDSize]
				msgBody := responsePayload[ed25519.SignatureSize+RequestIDSize:]
				if !bytes.Equal(responseRequestID, requestID) {
					logrus.Infof("skipping message due to requestID mismatch exp: %s, rec: %s\n", requestID, responseRequestID)
					continue loop
				}
				valid := ed25519.Verify(pkey, append(responseRequestID, msgBody...), responseSignature)
				if !valid {
					return nil, ErrInvalidSignature
				}
				return msgBody, nil
			}
		}
	}
}

// FetchDeviceState sends rqeuest to device with question about his device state
func (devmgmt Devmgmt) FetchDeviceState(ctx context.Context, device types.Device) (types.DeviceResponse, error) {
	publicKey, err := base64.StdEncoding.DecodeString(device.PublicKeyBase64)
	if err != nil {
		return types.DeviceResponse{}, fmt.Errorf("fail decode public key %s", err.Error())
	}

	deviceResponseJSON, err := devmgmt.PublishWithResponse(ctx, nil, ResponseTopic{
		Request:  fmt.Sprintf("%s/state/request", device.ID),
		Response: fmt.Sprintf("%s/state/response", device.ID),
	}, ed25519.PublicKey(publicKey))
	if err != nil {
		return types.DeviceResponse{}, err
	}

	var deviceResponse types.DeviceResponse
	if err = json.Unmarshal(deviceResponseJSON, &deviceResponse); err != nil {
		return types.DeviceResponse{}, fmt.Errorf("fail unmarshall device response %s", err.Error())
	}

	return deviceResponse, nil
}

// SendActionCommand sends action command and returns device response to it
func (devmgmt Devmgmt) SendActionCommand(
	ctx context.Context,
	device types.Device,
	command string,
	params map[string]interface{},
) (types.DeviceResponse, error) {
	deviceRequest := types.DeviceRequest{
		State:   params,
		Command: command,
	}
	deviceRequestJSON, err := json.Marshal(deviceRequest)
	if err != nil {
		return types.DeviceResponse{}, nil
	}
	publicKey, err := base64.StdEncoding.DecodeString(device.PublicKeyBase64)
	if err != nil {
		return types.DeviceResponse{}, fmt.Errorf("fail decode public key %s", err.Error())
	}
	deviceResponseJSON, err := devmgmt.PublishWithResponse(ctx, deviceRequestJSON, ResponseTopic{
		Request:  fmt.Sprintf("%s/command/request", device.ID),
		Response: fmt.Sprintf("%s/command/response", device.ID),
	}, ed25519.PublicKey(publicKey))

	if err != nil {
		return types.DeviceResponse{}, err
	}

	var deviceResponse types.DeviceResponse
	err = json.Unmarshal(deviceResponseJSON, &deviceResponse)
	if err != nil {
		return types.DeviceResponse{}, fmt.Errorf("invalid json %s", err.Error())
	}

	return deviceResponse, nil
}
