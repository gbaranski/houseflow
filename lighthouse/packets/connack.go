package packets

import (
	"fmt"
	"io"

	"github.com/gbaranski/houseflow/lighthouse/utils"
)

const (
	// ConnACKConnectionAccepted Connection accepted
	ConnACKConnectionAccepted byte = iota + 1

	// ConnACKOperationUnavailable The server cannot currently accept this operation
	ConnACKOperationUnavailable

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


// ReadExecutePayload reads ConnACKPayload from io.Reader
func ReadConnACKPayload(r io.Reader) (p ConnACKPayload, err error) {
  p.ReturnCode, err = utils.ReadByte(r)
	if err != nil {
		return p, fmt.Errorf("invalid returncode byte %s", err.Error())
	}

  return p, err
}

// WriteTo writes ConnACKPayload to io.Writer
func (p ConnACKPayload) WriteTo(w io.Writer) (n int64, err error) {
  k, err := utils.WriteByte(w, p.ReturnCode)
	return int64(k), err
}
