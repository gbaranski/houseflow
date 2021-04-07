package server

import (
	"fmt"
	"net"
  "bytes"

  "github.com/google/uuid"
	"github.com/gbaranski/houseflow/lighthouse/packets"
	"github.com/sirupsen/logrus"
)

// ListenTCP starts listening to incoming requests, this function is blocking
func (s *Server) ListenTCP() error {
	l, err := net.Listen("tcp", fmt.Sprintf("%s:%d", s.cfg.Hostname, s.cfg.Port))
	if err != nil {
		return err
	}
	defer l.Close()

	logrus.WithFields(logrus.Fields{
		"hostname": s.cfg.Hostname,
		"port":     s.cfg.Port,
	}).Info("Listening for incoming connections")

	for {
		conn, err := l.Accept()
		if err != nil {
      return fmt.Errorf("fail accepting connection: `%s`", err.Error())
		}
		go s.handleTCPConnection(conn)
	}
}


func (s *Server) waitConnectPacket(conn net.Conn) (clientID uuid.UUID, code byte)  {
	ptype, err := packets.ReadOpCode(conn)
	if err != nil {
		logrus.WithError(err).Error("fail read packet type")
		return uuid.Nil, packets.ConnACKMalformedPayload
	}

	if ptype != packets.OpCodeConnect {
		logrus.WithField("type", ptype).Error("Connection must start with CONNECT packet")
		return uuid.Nil, packets.ConnACKOperationUnavailable
	}

  connectPayload, err := packets.ReadConnectPayload(conn)
  if err != nil {
    return uuid.Nil, packets.ConnACKMalformedPayload
  }

  return connectPayload.ClientID, packets.ConnACKConnectionAccepted
}

func (s *Server) handleTCPConnection(conn net.Conn) {
	defer conn.Close()

  clientID, code := s.waitConnectPacket(conn)

  buf := bytes.NewBuffer([]byte{})
  _, err := packets.Packet{
    OpCode: packets.OpCodeConnACK,
    Payload: packets.ConnACKPayload{
    },
  }.WriteTo(buf)
  if err != nil {
    logrus.WithError(err).Error("failed writing to buf")
    return
  }
  _, err = buf.WriteTo(conn)
  if err != nil {
    logrus.WithError(err).Error("failed writing packet to connection")
    return
  }

  if code != packets.ConnACKConnectionAccepted {
    return
  }

  session := NewSession(conn, clientID)

  if err := s.SessionStore.Add(&session); err != nil {
    logrus.WithError(err).Error("failed adding session to store")
  }
	loge := logrus.WithFields(logrus.Fields{
		"clientID": session.ClientID,
		"ip":       session.conn.RemoteAddr().String(),
	})

	loge.Info("Started connection")

  if err := session.readLoop(); err != nil {
		loge.WithError(err).Error("fail readLoop()")
	}
}
