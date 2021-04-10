package tcp_server

import (
	"fmt"
	"net"
  "bytes"

	"github.com/gbaranski/houseflow/lighthouse/packets"
	"github.com/google/uuid"
	"github.com/sirupsen/logrus"
)

type Session struct {
	ClientID uuid.UUID
	conn     net.Conn
}

// NewSession waits for first CONNECT packet on connection and returns Session struct
func NewSession(conn net.Conn, clientID uuid.UUID) Session {
  return Session {
    ClientID: clientID,
    conn: conn,
  }
}

type packetHandler = func() (packets.Payload, error)

func (s *Session) readLoop() error {
	for {
		opcode, err := packets.ReadOpCode(s.conn)
		if err != nil {
			return fmt.Errorf("fail reading opcode: `%s`", err.Error())
		}
		logrus.WithField("opcode", opcode).Infof("Received packet")

		var handler packetHandler
		switch opcode {
		case packets.OpCodeExecuteResponse:
			handler = s.onExecuteResponse
		case packets.OpCodePing:
			handler = s.onPing
		case packets.OpCodePong:
			handler = s.onPong
		default:
			return fmt.Errorf("packet has invalid opcode: `%d`", opcode)
		}

    responsePayload, err := handler()
    if err != nil {
      return err
    }
    buf := bytes.NewBuffer([]byte{})
    if _, err := responsePayload.WriteTo(buf); err != nil {
      return err
    }
    if _, err := buf.WriteTo(s.conn); err != nil {
      return err
    }
	}
}

func (s *Session) SendExecute(payload packets.ExecutePayload) error {
  buf := bytes.NewBuffer([]byte{})
  packet := packets.Packet {
    OpCode: packets.OpCodeExecute,
    Payload: payload,
  }
  _, err := packet.WriteTo(buf)
  if err != nil {
    return fmt.Errorf("fail writing to buffer: %s", err.Error())
  }

  _, err = buf.WriteTo(s.conn)
  if err != nil {
    return fmt.Errorf("fail writing to conn: %s", err.Error())
  }

  return nil
}

//
// Execute response packet handler
//
func (s *Session) onExecuteResponse() (packets.Payload, error) {
	sp, err := packets.ReadExecuteResponsePayload(s.conn)
	if err != nil {
		return nil, err
	}

	logrus.WithFields(logrus.Fields{
		"clientID": s.ClientID,
		"msgID":    sp.ID,
		"state":     sp.State,
	}).Info("Received ExecuteResponse packet")

	return nil, nil
}

//
// PONG packet handler
//
func (s *Session) onPong() (packets.Payload, error) {
	pongp, err := packets.ReadPongPayload(s.conn)
	if err != nil {
		return nil, err
	}

	logrus.WithFields(logrus.Fields{
		"clientID": s.ClientID,
		"pongID":   pongp.ID,
	}).Info("Received PONG packet")

	return nil, nil
}

//
// PING packet handler
//
func (s *Session) onPing() (packets.Payload, error) {
	logrus.Infoln("Received Ping packet")

	pingp, err := packets.ReadPingPayload(s.conn)
	if err != nil {
		return nil, err
	}

	logrus.WithFields(logrus.Fields{
		"clientID": s.ClientID,
		"pingID":   pingp.ID,
	}).Info("Received PING packet")

  return packets.PongPayload{
    ID: pingp.ID,
  }, nil
}
