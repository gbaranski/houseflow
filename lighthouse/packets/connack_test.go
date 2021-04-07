package packets

import (
  "testing"
  "bytes"
  "math"
  "math/rand"
)

func TestConnACKPayload(t *testing.T) {
  payload := ConnACKPayload{
    ReturnCode: uint8(rand.Intn(math.MaxUint8)),
  }
  buf := bytes.NewBuffer([]byte{})
  _, err := payload.WriteTo(buf)
  if err != nil {
    t.Fatalf(err.Error())
  }

  newPayload, err := ReadConnACKPayload(buf)
  if err != nil {
    t.Fatalf("fail reading ConnACK payload: %s", err.Error())
  }

  if payload.ReturnCode != newPayload.ReturnCode {
    t.Fatalf("`payload.ReturnCode` does not match! Expected: `%d`, found: `%d`", payload.ReturnCode, newPayload.ReturnCode)
  }
}
