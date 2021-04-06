package client

import (
	"fmt"

	"github.com/gbaranski/houseflow/lighthouse/packets"
	log "github.com/sirupsen/logrus"
)

func (c Client) onPong() error {
	pongp, err := packets.ReadPongPayload(c.conn)
	if err != nil {
		return fmt.Errorf("fail parse payload %s", err.Error())
	}

	log.WithFields(log.Fields{
		"pongID": pongp.ID,
	}).Info("Received PONG packet")

	return nil
}
