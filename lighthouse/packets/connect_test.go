package packets

import (
  "testing"
  "bytes"
	"github.com/google/uuid"
)

func TestConnectPayload(t *testing.T) {
  payload := ConnectPayload{
    ClientID: uuid.New(),
  }
  buf := bytes.NewBuffer([]byte{})
  _, err := payload.WriteTo(buf)
  if err != nil {
    t.Fatalf(err.Error())
  }

  newPayload, err := ReadConnectPayload(buf)
  if err != nil {
    t.Fatalf("fail reading Connect payload: %s", err.Error())
  }

  if payload.ClientID != newPayload.ClientID {
    t.Fatalf("`payload.ClientID` does not match! Expected: `%d`, found: `%d`", payload.ClientID, newPayload.ClientID)
  }
}
