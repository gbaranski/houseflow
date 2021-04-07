package packets

import (
	"encoding/json"
	"fmt"
	"io"

	"github.com/gbaranski/houseflow/lighthouse/utils"
)


type ExecuteResponseStatus = byte

const (
  // Confirm that the command succeeded.
  ExecuteResponseStatusSuccess ExecuteResponseStatus = iota + 1

  // Command is enqueued but expected to succeed.
  ExecuteResponseStatusPending

  // Target device is in offline state or unreachable.
  ExecuteResponseStatusOffline

  // There is an issue or alert associated with a command. The command could succeed or fail. This status type is typically set when you want to send additional information about another connected device.
  ExecuteResponseStatusExceptions

  // Target device is unable to perform the command.
  ExecuteResponseStatusError
)

// ExecuteResponsePayload is payload of Execute Response packet
type ExecuteResponsePayload struct {
	// ID is unique identifier for each request, it is random 16 bit integer
	ID uint16

	// Status is status of the execute response
	Status ExecuteResponseStatus

	// State is state of the client
	State map[string]interface{}
}

// ReadExecutePayload reads ExecuteResponsePayload from io.Reader
func ReadExecuteResponsePayload(r io.Reader) (p ExecuteResponsePayload, err error) {
	p.ID, err = utils.Read16BitInteger(r)
	if err != nil {
		return p, fmt.Errorf("fail reading `ID` on `ExecuteResponsePayload`: `%s`", err.Error())
	}
	p.Status, err = utils.ReadByte(r)
	if err != nil {
		return p, fmt.Errorf("fail reading `Status` on `ExecuteResponsePayload`: `%s`", err.Error())
	}
	stateBytes, err := utils.ReadLengthPrefixedSlice(r)
	if err != nil {
		return p, fmt.Errorf("fail reading `state` on `ExecuteResponsePayload`: `%s`", err.Error())
	}
	err = json.Unmarshal(stateBytes, &p.State)
	if err != nil {
		return p, fmt.Errorf("fail unmarshalling JSON `state` for `ExecuteResponsePayload`: `%s`", err.Error())
	}

	return p, nil
}

// WriteTo writes ExecuteResponsePayload to io.Writer
func (p ExecuteResponsePayload) WriteTo(w io.Writer) (n int64, err error) {
	stateBytes, err := json.Marshal(p.State)
	if err != nil {
		return n, fmt.Errorf("fail marshalling to JSON property `State` on `ExecuteResponsePayload`: `%s`", err.Error())
	}
  k, err := utils.Write16BitInteger(w, p.ID)
  if err != nil {
		return n, fmt.Errorf("fail writing `ID`: `%s`", err.Error())
  }
  n += int64(k)

  k, err = utils.WriteByte(w, p.Status)
  if err != nil {
		return n, fmt.Errorf("fail writing `Status`: `%s`", err.Error())
  }
  n += int64(k)

  k, err = utils.WriteLengthPrefixedBytes(w, stateBytes)
  if err != nil {
		return n, fmt.Errorf("fail writing `state`: `%s`", err.Error())
  }
  n += int64(k)

  return n, nil
}
