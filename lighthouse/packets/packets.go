package packets

import (
	"bytes"
	"fmt"
	"io"

	"github.com/gbaranski/houseflow/lighthouse/utils"
)

type Payload interface {
	io.WriterTo
}

const (
	// OpCodeConnect - Client request to connect to Server operation code
	//
	// Direction: Client to Server
	OpCodeConnect OpCode = iota + 1

	// OpCodeConnACK - Connect acknowledgment operation code
	//
	// Direction: Server to Client
	OpCodeConnACK

	// OpCodeExecute - Execute operation code
	//
	// Direction: Server to Client
	OpCodeExecute

	// OpCodeExecute - Execute operation code
	//
	// Direction: Client to Server
	OpCodeExecuteResponse

	// OpCodePing - Ping request operation code
	//
	// Direction: Server to Client or Client to Server
	OpCodePing

	// OpCodePong - Ping acknowledgmenet operation code
	//
	// Direction: Server to Client or Client to Server
	OpCodePong
)

// OpCode defnes opcode of packet
type OpCode byte

// ReadOpCode reads packet type and returns it
func ReadOpCode(r io.Reader) (OpCode, error) {
	b, err := utils.ReadByte(r)
	return OpCode(b), err
}

// ReadPayloadSize reads size length, that must be called before reading payload
func ReadPayloadSize(r io.Reader) (uint16, error) {
	return utils.Read16BitInteger(r)
}

// Packet is type for LightMQ Packet
type Packet struct {
	OpCode  OpCode
	Payload Payload
}

func (p Packet) WriteTo(w io.Writer) (n int64, err error) {
	k, err := utils.WriteByte(w, byte(p.OpCode))
  n+=int64(k)
	if err != nil {
		return n, fmt.Errorf("error when writing OpCode: `%s`", err.Error())
	}

	buf := bytes.NewBuffer([]byte{})
	j, err := p.Payload.WriteTo(buf)
	if err != nil {
		return n, fmt.Errorf("error when writing Payload: `%s`", err.Error())
	}

  k, err = utils.Write32BitInteger(w, uint32(j))
  n += int64(k)
	if err != nil {
		return n, fmt.Errorf("error when writing payload length: `%s`", err.Error())
	}

  j, err = buf.WriteTo(w)
  n += j
	if err != nil {
		return n, fmt.Errorf("error when writing payload: `%s`", err.Error())
	}

  return 
}
