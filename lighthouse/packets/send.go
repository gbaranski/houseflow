package packets

import (
	"encoding/binary"
	"fmt"
	"io"

	"github.com/gbaranski/houseflow/lighthouse/utils"
)

// SendPayload is payload of SEND packet
type SendPayload struct {
	ID    uint16
	Flags byte
	Data  []byte
}

// ReadSendPayload reads SendPayload from io.Reader
func ReadSendPayload(r io.Reader) (sp SendPayload, err error) {
	len, err := utils.Read16BitInteger(r)
	if err != nil {
		return sp, err
	}
	sp.ID, err = utils.Read16BitInteger(r)
	if err != nil {
		return sp, err
	}
	sp.Flags, err = utils.ReadByte(r)
	if err != nil {
		return sp, err
	}

	sp.Data = make([]byte, len-3)
	_, err = r.Read(sp.Data)
	if err != nil {
		return sp, fmt.Errorf("fail read data %s", err.Error())
	}

	return sp, nil
}

// Bytes convert payload to byte slice
func (p SendPayload) Bytes() []byte {
	msgID := make([]byte, 2)
	binary.BigEndian.PutUint16(msgID, p.ID)

	// Optimize size
	b := make([]byte, 0)
	b = append(b, msgID...)
	b = append(b, p.Flags)
	b = append(b, p.Data...)

	return b
}
