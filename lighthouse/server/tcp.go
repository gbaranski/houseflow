package server

import (
	"fmt"
	"net"

	"github.com/gbaranski/houseflow/lighthouse/packets"
	log "github.com/sirupsen/logrus"
)

// ListenTCP starts listening to incoming requests, this function is blocking
func (b *Server) ListenTCP() error {
	l, err := net.Listen("tcp", fmt.Sprintf("%s:%d", b.cfg.Hostname, b.cfg.Port))
	if err != nil {
		return err
	}
	log.WithFields(log.Fields{
		"hostname": b.cfg.Hostname,
		"port":     b.cfg.Port,
	}).Info("Listening for incoming LightMQ connections")
	defer l.Close()
	for {
		conn, err := l.Accept()
		if err != nil {
			return fmt.Errorf("fail accepting connection %s", err.Error())
		}
		go b.handleTCPConnection(conn)
	}
}

func (b *Server) handleTCPConnection(conn net.Conn) {
	defer conn.Close()

	ptype, err := packets.ReadOpCode(conn)
	if err != nil {
		log.WithError(err).Error("fail read packet type")
		return
	}

	if ptype != packets.OpCodeConnect {
		log.WithField("type", ptype).Error("Connection must start with CONNECT packet")
		return
	}
	client, err := b.onConnect(conn)
	if err != nil {
		log.WithError(err).Error("fail handle connection")
		return
	}
	go b.ClientStore.Add(client)
	loge := log.WithFields(log.Fields{
		"clientID": client.ID,
		"ip":       client.IPAddress.String(),
	})

	loge.Info("Started connection")

	err = b.readLoop(conn, client)
	if err != nil {
		loge.WithError(err).Error("fail readLoop()")
	}
}
