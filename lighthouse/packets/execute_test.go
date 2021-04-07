package packets

import (
  "testing"
  "bytes"
  "math"
  "math/rand"
  "reflect"
)

func TestExecutePayload(t *testing.T) {
  payload := ExecutePayload{
    ID: uint16(rand.Intn(math.MaxUint16)),
    Command: 123,
    Params: map[string]interface{}{
      "dsahasdha": 123,
    },
  }
  buf := bytes.NewBuffer([]byte{})
  _, err := payload.WriteTo(buf)
  if err != nil {
    t.Fatalf(err.Error())
  }

  newPayload, err := ReadExecutePayload(buf)
  if err != nil {
    t.Fatalf("fail reading execute payload: %s", err.Error())
  }

  if payload.ID != newPayload.ID {
    t.Fatalf("`payload.ID` does not match! Expected: `%d`, found: `%d`", payload.ID, newPayload.ID)
  }
  if payload.Command != newPayload.Command {
    t.Fatalf("`payload.Command` does not match! Expected: `%d`, found: `%d`", payload.Command, newPayload.Command)
  }
  if reflect.DeepEqual(payload.Params, newPayload.Params) {
    t.Fatalf("`payload.Params` does not match! Expected: `%+v`, found: `%+v`", payload.Params, newPayload.Params)
  }
}
