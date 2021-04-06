package server

import (
	"fmt"

	"github.com/gbaranski/houseflow/lighthouse/packets"
	log "github.com/sirupsen/logrus"
)

// onPing should be executed on PING packet
func (b *Server) onPing(p packet) error {
	pingp, err := packets.ReadPingPayload(p)
	if err != nil {
		return err
	}

	log.WithFields(log.Fields{
		"clientID": p.Client.ID,
		"pingID":   pingp.ID,
	}).Info("Received PING packet")

	pongPacket := packets.Packet{
		OpCode: packets.OpCodePong,
		Payload: packets.PongPayload{
			ID: pingp.ID,
		}.Bytes(),
	}.Bytes()
	if err != nil {
		return fmt.Errorf("fail convert connack packet to bytes %s", err.Error())
	}

	_, err = p.Write(pongPacket)
	if err != nil {
		return fmt.Errorf("fail send pong %s", err.Error())
	}

	log.WithFields(log.Fields{
		"clientID": p.Client.ID,
		"pongID":   pingp.ID,
	}).Info("Sent PONG packet")

	return err
}
