package client

import (
	"fmt"

	"github.com/gbaranski/houseflow/lighthouse/packets"
	log "github.com/sirupsen/logrus"
)

func (c Client) onPing() error {
	pingPayload, err := packets.ReadPingPayload(c.conn)
	if err != nil {
		return fmt.Errorf("fail parse payload %s", err.Error())
	}

	pongPayload := packets.PongPayload{
		ID: pingPayload.ID,
	}
	_, err = c.conn.Write(pongPayload.Bytes())
	if err != nil {
		return fmt.Errorf("fail send pong %s", err.Error())
	}

	log.WithFields(log.Fields{
		"pongID": pongPayload.ID,
	}).Info("Sent PONG packet")

	return nil
}
