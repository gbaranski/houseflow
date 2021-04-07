package packets

import (
  "testing"
  "bytes"
  "math"
  "math/rand"
)

func TestPongPayload(t *testing.T) {
  payload := PongPayload{
    ID: uint16(rand.Intn(math.MaxUint16)),
  }
  buf := bytes.NewBuffer([]byte{})
  _, err := payload.WriteTo(buf)
  if err != nil {
    t.Fatalf(err.Error())
  }

  newPayload, err := ReadPongPayload(buf)
  if err != nil {
    t.Fatalf("fail reading execute payload: %s", err.Error())
  }

  if payload.ID != newPayload.ID {
    t.Fatalf("`payload.ID` does not match! Expected: `%d`, found: `%d`", payload.ID, newPayload.ID)
  }
}
