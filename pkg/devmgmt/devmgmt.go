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
func (d Devmgmt) PublishWithResponse(ctx context.Context, body []byte, topic ResponseTopic, pkey ed25519.PublicKey) ([]byte, error) {
	msgc := make(chan paho.Message)

	if token := d.Client.Subscribe(topic.Response, 0, func(c paho.Client, m paho.Message) {
		msgc <- m
	}); token.Wait() && token.Error() != nil {
		return nil, token.Error()
	}
	defer func() {
		d.Client.Unsubscribe(topic.Response)
	}()
	requestID, err := utils.NewRequestID(RequestIDSize)
	if err != nil {
		return nil, fmt.Errorf("fail gen requestID %s", err.Error())
	}

	p := make([]byte, ed25519.SignatureSize+RequestIDSize+len(body))
	copy(p[0:ed25519.SignatureSize], ed25519.Sign(d.opts.ServerPrivateKey, append(requestID, body...)))
	copy(p[ed25519.SignatureSize:ed25519.SignatureSize+RequestIDSize], requestID)
	copy(p[ed25519.SignatureSize+RequestIDSize:], body)

	if token := d.Client.Publish(topic.Request, 0, false, p); token.Wait() && token.Error() != nil {
		return nil, token.Error()
	}

loop:
	for {
		select {
		case <-ctx.Done():
			return nil, ErrDeviceTimeout
		case msg, ok := <-msgc:
			{
				if !ok {
					continue loop
				}
				msgPayload := msg.Payload()
				msgSig := msgPayload[0:ed25519.SignatureSize]
				msgRequestID := msgPayload[ed25519.SignatureSize : ed25519.SignatureSize+RequestIDSize]
				msgBody := msgPayload[ed25519.SignatureSize+RequestIDSize:]
				if !bytes.Equal(msgRequestID, requestID) {
					continue loop
				}
				valid := ed25519.Verify(pkey, msgBody, msgSig)
				if !valid {
					return nil, fmt.Errorf("invalid message signature")
				}
				return msgBody, nil
			}
		}
	}
}

// FetchDeviceState sends rqeuest to device with question about his device state
func (d Devmgmt) FetchDeviceState(ctx context.Context, device types.Device) (types.DeviceResponse, error) {
	res, err := d.PublishWithResponse(ctx, nil, ResponseTopic{
		Request:  fmt.Sprintf("%s/state/request", device.ID),
		Response: fmt.Sprintf("%s/state/response", device.ID),
	}, ed25519.PublicKey(device.PublicKey))
	if err != nil {
		return types.DeviceResponse{}, err
	}

	var parsedResponse types.DeviceResponse
	if err = json.Unmarshal(res, &parsedResponse); err != nil {
		return types.DeviceResponse{}, fmt.Errorf("fail unmarshall device resp %s", err.Error())
	}

	return parsedResponse, nil
}

// SendActionCommand sends action command and returns device response to it
func (d Devmgmt) SendActionCommand(
	ctx context.Context,
	device types.Device,
	command string,
	params map[string]interface{},
) (types.DeviceResponse, error) {
	req := types.DeviceRequest{
		State:   params,
		Command: command,
	}
	r, err := json.Marshal(req)
	if err != nil {
		return types.DeviceResponse{}, nil
	}
	b, err := d.PublishWithResponse(ctx, r, ResponseTopic{
		Request:  fmt.Sprintf("%s/request", device.ID),
		Response: fmt.Sprintf("%s/response", device.ID),
	}, ed25519.PublicKey(device.PublicKey))

	if err != nil {
		return types.DeviceResponse{}, err
	}

	var res types.DeviceResponse
	err = json.Unmarshal(b, &res)
	if err != nil {
		return types.DeviceResponse{}, fmt.Errorf("invalid json %s", err.Error())
	}

	return res, nil
}
