package packets

import (
	"io"

	"github.com/gbaranski/houseflow/lighthouse/utils"
)

const (
	// OpCodeConnect - Client request to connect to Server operation code
	//
	// Direction: Client to Server
	OpCodeConnect OpCode = iota + 1

	// OpCodeConnACK - Connect acknowledgment operation code
	//
	// Direction: Server to Client
	OpCodeConnACK

	// OpCodeSend - Send message operation code
	//
	// Direction: Server to Client or Client to Server
	OpCodeSend

	// OpCodeSendResponse - Send Response operation code
	//
	// Direction: Server to Client or Client to Server
	OpCodeSendResponse

	// OpCodePing - Ping request operation code
	//
	// Direction: Server to Client or Client to Server
	OpCodePing

	// OpCodePong - Ping acknowledgmenet operation code
	//
	// Direction: Server to Client or Client to Server
	OpCodePong
)

// Payload is type for LightMQ Payload
type Payload []byte

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

// Bytes converts packet to bytes which can be directly sent
func (p Packet) Bytes() []byte {
	length := 1 + // opcode
		2 + // payload length in two bytes(uint16)
		len(p.Payload) // length of the actual payload

	b := make([]byte, length)
	b[0] = byte(p.OpCode)
	b[1] = byte(len(p.Payload) >> 8)
	b[2] = byte(len(p.Payload))

	copy(b[3:], p.Payload)

	return b
}
