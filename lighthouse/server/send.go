package server

import (
	"github.com/gbaranski/houseflow/lighthouse/packets"
	log "github.com/sirupsen/logrus"
)

// onSend should be executed on SEND packet
func (b *Server) onSend(p packet) error {
	sp, err := packets.ReadSendPayload(p)
	if err != nil {
		return err
	}

	log.WithFields(log.Fields{
		"clientID": p.Client.ID,
		"msgID":    sp.ID,
		"data":     string(sp.Data),
	}).Info("Received SEND packet")

	return nil
}
