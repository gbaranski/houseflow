package packets

import (
	"encoding/binary"
	"fmt"
	"io"

	"github.com/gbaranski/houseflow/lighthouse/utils"
)

// SendResponsePayload is payload of SENDRESP packet
type SendResponsePayload struct {
	ID   uint16
	Data []byte
}

// ReadSendResponsePayload reads SendResponsePayload from io.Reader
func ReadSendResponsePayload(r io.Reader) (sp SendResponsePayload, err error) {
	len, err := utils.Read16BitInteger(r)
	if err != nil {
		return sp, err
	}
	idBytes := make([]byte, 2)
	_, err = r.Read(idBytes)
	if err != nil {
		return sp, fmt.Errorf("fail read ID bytes %s", err.Error())
	}
	sp.ID = binary.BigEndian.Uint16(idBytes)

	sp.Data = make([]byte, len-3)
	_, err = r.Read(sp.Data)
	if err != nil {
		return sp, fmt.Errorf("fail read data %s", err.Error())
	}

	return sp, nil
}

// Bytes convert payload to byte slice
func (p SendResponsePayload) Bytes() []byte {
	msgID := make([]byte, 2)
	binary.BigEndian.PutUint16(msgID, p.ID)

	// Optimize size
	b := make([]byte, 0)
	b = append(b, msgID...)
	b = append(b, p.Data...)

	return b
}
