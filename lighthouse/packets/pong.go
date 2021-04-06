package packets

import (
	"fmt"
	"io"

	"github.com/gbaranski/houseflow/lighthouse/utils"
)

// PongPayload is payload for PONG Control packet
type PongPayload struct {
	ID uint16
}

// ReadPongPayload reads pong payload from io.Reader
func ReadPongPayload(r io.Reader) (PongPayload, error) {
	length, err := utils.Read16BitInteger(r)
	if err != nil {
		return PongPayload{}, fmt.Errorf("fail read payload len")
	}
	if length != 2 {
		return PongPayload{}, fmt.Errorf("invlaid length: %d", length)
	}
	id, err := utils.Read16BitInteger(r)
	if err != nil {
		return PongPayload{}, err
	}
	return PongPayload{
		ID: id,
	}, nil
}

// Bytes convert PongPayload to bytes
func (p PongPayload) Bytes() (b []byte) {
	b = make([]byte, 2)
	b[0] = byte(p.ID >> 8)
	b[1] = byte(p.ID)

	return b
}
