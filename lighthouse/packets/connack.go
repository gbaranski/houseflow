package packets

import (
	"fmt"
	"io"

	"github.com/gbaranski/houseflow/lighthouse/utils"
)

const (
	// ConnACKConnectionAccepted Connection accepted
	ConnACKConnectionAccepted byte = iota + 1

	// ConnACKUnsupportedProtocol The Server does not support the level of the LightMQ protocol requested by the Client
	ConnACKUnsupportedProtocol

	// ConnACKServerUnavailable The Network Connection has been made but the LightMQ service is unavailable
	ConnACKServerUnavailable

	// ConnACKMalformedPayload Malformed payload
	ConnACKMalformedPayload

	// ConnACKUnauthorized The Client is not authorized to connect
	ConnACKUnauthorized
)

// ConnACKPayload is the packet sent by the Server in response to a CONNECT Packet received from a Client. The first packet sent from the Server to the Client MUST be a CONNACK Packet
type ConnACKPayload struct {
	// e.g ConnACKConnectionAccepted
	ReturnCode byte
}

// Bytes converts ConnACK to bytes
func (c ConnACKPayload) Bytes() []byte {
	b := make([]byte, 1)
	b[0] = c.ReturnCode
	return b
}

// ReadConnACKPayload reads ConnACK packet payload
func ReadConnACKPayload(r io.Reader) (ConnACKPayload, error) {
	length, err := utils.Read16BitInteger(r)
	if err != nil {
		return ConnACKPayload{}, fmt.Errorf("unable read length %s", err.Error())
	}
	if length != 1 {
		return ConnACKPayload{}, fmt.Errorf("invalid length %d", length)
	}
	p, err := utils.ReadByte(r)
	if err != nil {
		return ConnACKPayload{}, fmt.Errorf("invalid returncode byte %s", err.Error())
	}
	return ConnACKPayload{
		ReturnCode: p,
	}, nil
}
