package client

import (
	"context"
	"fmt"
	"math"
	"math/rand"
	"net"
	"time"

	"github.com/gbaranski/houseflow/lighthouse/packets"
	"github.com/sirupsen/logrus"
)

// PacketChannels is struct which contains of channels for packets
type packetChannels struct {
	ConnACK  chan packets.ConnACKPayload
	Ping     chan packets.PingPayload
	Pong     chan packets.PongPayload
	Send     chan packets.SendPayload
	SendResp chan packets.SendResponsePayload
}

// Client ...
type Client struct {
	cfg         Config
	conn        net.Conn
	channels    packetChannels
	ConnCloseCh chan struct{}
}

// New creates new client
func New(cfg Config) Client {
	return Client{
		cfg: cfg,
		channels: packetChannels{
			ConnACK: make(chan packets.ConnACKPayload),
			Ping:    make(chan packets.PingPayload),
			Pong:    make(chan packets.PongPayload),
		},
	}
}

// Connect connects to the specified host with specified port
func (c *Client) Connect(ctx context.Context) error {
	var err error
	c.conn, err = net.Dial("tcp", fmt.Sprintf("%s:%d", c.cfg.Hostname, c.cfg.Port))
	if err != nil {
		return err
	}

	go c.ReadLoop()

	p := packets.Packet{
		OpCode: packets.OpCodeConnect,
		Payload: packets.ConnectPayload{
			ClientID: c.cfg.ClientID,
		}.Bytes(),
	}.Bytes()
	if err != nil {
		return fmt.Errorf("fail convert connect packet to bytes %s", err.Error())
	}
	_, err = c.conn.Write(p)
	if err != nil {
		return fmt.Errorf("fail write CONNECT packet %s", err.Error())
	}

	select {
	case <-ctx.Done():
		return fmt.Errorf("context finished before connecting")
	case c := <-c.channels.ConnACK:
		if c.ReturnCode != packets.ConnACKConnectionAccepted {
			return fmt.Errorf("unexpected return code 0x%x", c.ReturnCode)
		}
		return nil
	}
}

func (c Client) handleWithOpCode(opcode packets.OpCode) error {
	switch opcode {
	case packets.OpCodeConnACK:
		payload, err := packets.ReadConnACKPayload(c.conn)
		if err != nil {
			return fmt.Errorf("fail read ConnACK payload, err: %s", err.Error())
		}
		c.channels.ConnACK <- payload
	case packets.OpCodePing:
		payload, err := packets.ReadPingPayload(c.conn)
		if err != nil {
			return fmt.Errorf("fail read Ping payload, err: %s", err.Error())
		}
		c.channels.Ping <- payload
	case packets.OpCodePong:
		payload, err := packets.ReadPongPayload(c.conn)
		if err != nil {
			return fmt.Errorf("fail read Pong payload, err: %s", err.Error())
		}
		c.channels.Pong <- payload
	case packets.OpCodeSendResponse:
		payload, err := packets.ReadSendResponsePayload(c.conn)
		if err != nil {
			return fmt.Errorf("fail read SendResponse payload, err: %s", err.Error())
		}
		c.channels.SendResp <- payload
	default:
		return fmt.Errorf("Unhandleable opcode")
	}
	return nil
}

// ReadLoop reads all data from connection in loop
func (c Client) ReadLoop() {
	for {
		opcode, err := packets.ReadOpCode(c.conn)
		if err != nil {
			c.ConnCloseCh <- struct{}{}
			logrus.Errorf("fail read operation code %s", err.Error())
			return
		}
		err = c.handleWithOpCode(opcode)
		if err != nil {
			logrus.WithField("opcode", opcode).Errorf(err.Error())
		}
	}
}

// Send sends a data
func (c Client) Send(data []byte) error {
	payload := packets.SendPayload{
		ID:    uint16(rand.Intn(math.MaxInt16)),
		Flags: 0,
		Data:  data,
	}.Bytes()
	packet := packets.Packet{
		OpCode:  packets.OpCodeSend,
		Payload: payload,
	}.Bytes()

	_, err := c.conn.Write(packet)

	return err
}

// SendWithResponse sends a data and waits for response
func (c Client) SendWithResponse(ctx context.Context, data []byte) (packets.SendResponsePayload, error) {
	payload := packets.SendPayload{
		ID:    uint16(rand.Intn(math.MaxInt16)),
		Flags: 0,
		Data:  data,
	}
	packet := packets.Packet{
		OpCode:  packets.OpCodeSend,
		Payload: payload.Bytes(),
	}.Bytes()
	_, err := c.conn.Write(packet)
	if err != nil {
		return packets.SendResponsePayload{}, err
	}

	for {
		select {
		case <-ctx.Done():
			return packets.SendResponsePayload{}, fmt.Errorf("timeout waiting for response")
		case p := <-c.channels.SendResp:
			if p.ID != payload.ID {
				continue
			}
			return p, nil
		}
	}
}

// Ping sends PING packet, returns ID of ping
func (c Client) Ping(ctx context.Context) (uint16, error) {
	payload := packets.PingPayload{
		ID: uint16(rand.Intn(math.MaxUint16)),
	}
	packet := packets.Packet{
		OpCode:  packets.OpCodePing,
		Payload: payload.Bytes(),
	}.Bytes()

	sendTime := time.Now()
	_, err := c.conn.Write(packet)
	if err != nil {
		return payload.ID, fmt.Errorf("fail write packet %s", err.Error())
	}

	select {
	case <-ctx.Done():
		return payload.ID, fmt.Errorf("context finished before PONG packet")
	case <-c.channels.Pong:
		logrus.Infof("Received pong after %s", time.Since(sendTime).String())
		return payload.ID, nil
	}
}
