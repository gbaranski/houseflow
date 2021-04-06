package packets

import (
	"bytes"
	"math"
	"math/rand"
	"testing"
)

func TestPingPayload(t *testing.T) {
	payload := PingPayload{
		ID: uint16(rand.Intn(math.MaxUint16)),
	}
	b := make([]byte, 0)
	b = append(b, 0) // MSB
	b = append(b, 2) // LSB
	b = append(b, payload.Bytes()...)

	readenPayload, err := ReadPingPayload(bytes.NewReader(b))
	if err != nil {
		t.Fatalf(err.Error())
	}
	if readenPayload.ID != payload.ID {
		t.Fatalf("Unexpected ID: %d, expected: %d", readenPayload.ID, payload.ID)
	}
}
