package packets

import (
	"fmt"
	"io"

	"github.com/gbaranski/houseflow/lighthouse/utils"
)

// PingPayload is payload for PING Control packet
type PingPayload struct {
	ID uint16
}

// ReadPingPayload reads ping payload from io.Reader
func ReadPingPayload(r io.Reader) (PingPayload, error) {
	length, err := utils.Read16BitInteger(r)
	if err != nil {
		return PingPayload{}, fmt.Errorf("fail read payload len")
	}
	if length != 2 {
		return PingPayload{}, fmt.Errorf("invlaid length: %d", length)
	}
	id, err := utils.Read16BitInteger(r)
	if err != nil {
		return PingPayload{}, err
	}
	return PingPayload{
		ID: id,
	}, nil
}

// Bytes convert PingPayload to bytes
func (p PingPayload) Bytes() (b []byte) {
	b = make([]byte, 2)
	b[0] = byte(p.ID >> 8)
	b[1] = byte(p.ID)

	return b
}
