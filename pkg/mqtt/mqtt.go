package mqtt

import (
	"fmt"
	"log"
	"os"
	"time"

	paho "github.com/eclipse/paho.mqtt.golang"
)

type MQTTOptions struct {
	// ClientID, required
	ClientID string

	// Default: "tcp://emqx:1883/mqtt"
	BrokerURL string

	// KeepAlive
	//
	// Default: 30s
	KeepAlive time.Duration

	// PingTimeout
	//
	// Default: 5s
	PingTimeout time.Duration
}

// Parses options to the defaults
func (opts *MQTTOptions) Parse() {
	if opts.BrokerURL == "" {
		opts.BrokerURL = "tcp://emqx:1883/mqtt"
	}
	if opts.KeepAlive == 0 {
		opts.KeepAlive = time.Second * 30
	}

	if opts.PingTimeout == 0 {
		opts.PingTimeout = time.Second * 5
	}
}

type MQTT struct {
	client paho.Client
	opts   MQTTOptions
}

func NewMQTT(opts MQTTOptions) MQTT {
	paho.ERROR = log.New(os.Stdout, "[ERROR] ", 0)
	paho.CRITICAL = log.New(os.Stdout, "[CRIT] ", 0)
	paho.WARN = log.New(os.Stdout, "[WARN]  ", 0)
	// mqtt.DEBUG = log.New(os.Stdout, "[DEBUG] ", 0)

	// Add there some kind of password soon
	copts := paho.
		NewClientOptions().
		AddBroker(opts.BrokerURL).
		SetClientID(opts.ClientID).
		SetKeepAlive(opts.KeepAlive).
		SetPingTimeout(opts.PingTimeout)

	c := paho.NewClient(copts)
	if token := c.Connect(); token.Wait() && token.Error() != nil {
		panic(token.Error())
	}
	return MQTT{
		client: c,
		opts:   opts,
	}
}
