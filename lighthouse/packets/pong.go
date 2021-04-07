package packets

import (
	"io"

	"github.com/gbaranski/houseflow/lighthouse/utils"
)

// PongPayload is payload for PONG packet
type PongPayload struct {
	ID uint16
}

// ReadPongPayload reads pong payload from io.Reader
func ReadPongPayload(r io.Reader) (p PongPayload, err error) {
	id, err := utils.Read16BitInteger(r)
	if err != nil {
    return p, err
	}

	return PongPayload{
		ID: id,
	}, nil
}

// WriteTo writes PongPayload to io.Writer
func (p PongPayload) WriteTo(w io.Writer) (n int64, err error) {
  k, err := utils.Write16BitInteger(w, p.ID)

  return int64(k), err
}
