package packets

import (
	"fmt"
	"io"

	"github.com/gbaranski/houseflow/lighthouse/utils"
)

// ConnectPayload ...
type ConnectPayload struct {
	ClientID string
}

// ReadConnectPayload ...
func ReadConnectPayload(r io.Reader) (cp ConnectPayload, err error) {
	clientIDSize, err := utils.ReadByte(r)
	if err != nil {
		return cp, fmt.Errorf("fail read clientID len %s", err.Error())
	}
	clientID := make([]byte, clientIDSize)
	n, err := r.Read(clientID)
	if err != nil {
		return cp, fmt.Errorf("fail read clientID %s", err.Error())
	}
	if n != int(clientIDSize) {
		return cp, fmt.Errorf("invalid clientID size n: %d, exp %d", n, clientIDSize)
	}
	cp.ClientID = string(clientID)

	return cp, nil
}

// Bytes converts ConnectPayload to payload bytes
func (cp ConnectPayload) Bytes() []byte {
	p := make([]byte, 1+len(cp.ClientID))
	p[0] = byte(len(cp.ClientID))
	for i, c := range cp.ClientID {
		p[i+1] = byte(c)
	}
	return p

}
