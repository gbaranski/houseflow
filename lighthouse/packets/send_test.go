package packets

import (
	"bytes"
	"math"
	"math/rand"
	"testing"
)

func TestSendPayload(t *testing.T) {
	payload := SendPayload{
		ID:    uint16(rand.Intn(math.MaxUint16)),
		Flags: 0,
		Data:  []byte("Hello world"),
	}
	payloadb := payload.Bytes()
	b := make([]byte, 2+len(payloadb))
	b[0] = byte(len(payloadb) >> 8)
	b[1] = byte(len(payloadb))
	copy(b[2:], payloadb)
	sp, err := ReadSendPayload(bytes.NewReader(b))
	if err != nil {
		t.Fatalf(err.Error())
	}
	if !bytes.Equal(payload.Data, sp.Data) {
		t.Fatalf("Unexpected data: %v, expected: %v", sp.Data, payload.Data)
	}
	if payload.Flags != sp.Flags {
		t.Fatalf("unexpected flags: %x, expected: %x", sp.Flags, payload.Flags)
	}
	if payload.ID != sp.ID {
		t.Fatalf("invalid msg ID: %x, expected: %x", sp.ID, payload.ID)
	}

}
