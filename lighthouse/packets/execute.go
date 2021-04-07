package packets

import (
	"encoding/json"
	"fmt"
	"io"

	"github.com/gbaranski/houseflow/lighthouse/utils"
)

// ExecutePayload is payload of EXECUTE packet
type ExecutePayload struct {
	// ID is unique identifier for each request, it is random 16 bit integer
	ID uint16

	// Command is specific code for command
	Command uint32

	// Params are execute parameters, it will be encoded into JSON later on
	Params map[string]interface{}
}

// ReadExecutePayload reads ExecutePayload from io.Reader
func ReadExecutePayload(r io.Reader) (p ExecutePayload, err error) {
	p.ID, err = utils.Read16BitInteger(r)
	if err != nil {
		return p, fmt.Errorf("fail reading `ID` on `ExecutePayload`: `%s`", err.Error())
	}
	p.Command, err = utils.Read32BitInteger(r)
	if err != nil {
		return p, fmt.Errorf("fail reading `command` on `ExecutePayload`: `%s`", err.Error())
	}
	paramsBytes, err := utils.ReadLengthPrefixedSlice(r)
	if err != nil {
		return p, fmt.Errorf("fail reading `params` on `ExecutePayload`: `%s`", err.Error())
	}
	err = json.Unmarshal(paramsBytes, &p.Params)
	if err != nil {
		return p, fmt.Errorf("fail unmarshalling JSON `params` for `ExecutePayload`: `%s`", err.Error())
	}

	return p, nil
}

// WriteTo writes ExecutePayload to io.Writer
func (p ExecutePayload) WriteTo(w io.Writer) (n int64, err error) {
	paramsBytes, err := json.Marshal(p.Params)
	if err != nil {
		return n, fmt.Errorf("fail marshalling to JSON property `Params` on `ExecutePayload`: `%s`", err.Error())
	}
  k, err := utils.Write16BitInteger(w, p.ID)
  if err != nil {
		return n, fmt.Errorf("fail writing `ID`: `%s`", err.Error())
  }
  n += int64(k)

  k, err = utils.Write32BitInteger(w, p.Command)
  if err != nil {
		return n, fmt.Errorf("fail writing `Command`: `%s`", err.Error())
  }
  n += int64(k)

  k, err = utils.WriteLengthPrefixedBytes(w, paramsBytes)
  if err != nil {
		return n, fmt.Errorf("fail writing `params`: `%s`", err.Error())
  }
  n += int64(k)

  return n, nil
}
