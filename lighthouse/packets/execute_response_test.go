package packets

import (
  "testing"
  "bytes"
  "math"
  "math/rand"
  "reflect"
)

func TestExecuteResponsePayload(t *testing.T) {
  payload := ExecuteResponsePayload{
    ID: uint16(rand.Intn(math.MaxUint16)),
    Status: ExecuteResponseStatusSuccess,
    State: map[string]interface{}{
      "dsahasdha": 123,
    },
  }
  buf := bytes.NewBuffer([]byte{})
  _, err := payload.WriteTo(buf)
  if err != nil {
    t.Fatalf(err.Error())
  }

  newPayload, err := ReadExecuteResponsePayload(buf)
  if err != nil {
    t.Fatalf("fail reading execute response payload: %s", err.Error())
  }

  if payload.ID != newPayload.ID {
    t.Fatalf("`payload.ID` does not match! Expected: `%d`, found: `%d`", payload.ID, newPayload.ID)
  }
  if payload.Status != newPayload.Status {
    t.Fatalf("`payload.Status` does not match! Expected: `%d`, found: `%d`", payload.Status, newPayload.Status)
  }
  if reflect.DeepEqual(payload.State, newPayload.State) {
    t.Fatalf("`payload.State` does not match! Expected: `%+v`, found: `%+v`", payload.State, newPayload.State)
  }
}
