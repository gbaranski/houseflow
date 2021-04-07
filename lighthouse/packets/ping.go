package packets

import (
	"io"

	"github.com/gbaranski/houseflow/lighthouse/utils"
)

// PingPayload is payload for PING packet
type PingPayload struct {
	ID uint16
}

// ReadPingPayload reads pong payload from io.Reader
func ReadPingPayload(r io.Reader) (p PingPayload, err error) {
	id, err := utils.Read16BitInteger(r)
	if err != nil {
    return p, err
	}

	return PingPayload{
		ID: id,
	}, nil
}

// WriteTo writes PingPayload to io.Writer
func (p PingPayload) WriteTo(w io.Writer) (n int64, err error) {
  k, err := utils.Write16BitInteger(w, p.ID)

  return int64(k), err
}
