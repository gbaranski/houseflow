package packets

import (
	"fmt"
	"io"

	"github.com/gbaranski/houseflow/lighthouse/utils"
	"github.com/google/uuid"
)

const ClientIDSize uint8 = 16

// ConnectPayload ...
type ConnectPayload struct {
	ClientID uuid.UUID
}

// ReadConnectPayload reads ConnectPayload from io.Reader
func ReadConnectPayload(r io.Reader) (p ConnectPayload, err error) {
	p.ClientID, err = utils.ReadUUID(r)
	if err != nil {
		return p, fmt.Errorf("fail read clientID %s", err.Error())
	}

	return p, err
}

// WriteTo writes ExecutePayload to io.Writer
func (p ConnectPayload) WriteTo(w io.Writer) (n int64, err error) {
  k, err := utils.WriteUUID(w, p.ClientID)
  if err != nil {
		return n, fmt.Errorf("fail writing `ClientID`: `%s`", err.Error())
  }
  n += int64(k)

  return n, nil
}
