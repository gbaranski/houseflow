package fulfillment

import (
	"encoding/json"

	"github.com/gbaranski/houseflow/pkg/devmgmt"
	"github.com/gbaranski/houseflow/pkg/fulfillment"
)

func (f *Fulfillment) executeCommand(
	r intentRequest,
	targetDevice fulfillment.QueryRequestPayloadDevice,
	execution fulfillment.ExecuteRequestExecution,
) fulfillment.ExecuteResponseCommands {

	perms, err := f.db.GetUserDevicePermissions(r.r.Context(), r.user.ID, targetDevice.ID)
	if err != nil {
		return fulfillment.ExecuteResponseCommands{
			Status:    "ERROR",
			ErrorCode: "hardError",
		}
	}
	if !perms.Execute {
		return fulfillment.ExecuteResponseCommands{
			Status:    fulfillment.StatusError,
			ErrorCode: "authFailure",
		}
	}
	device, err := f.db.GetDeviceByID(r.r.Context(), targetDevice.ID)
	if err != nil {
		return fulfillment.ExecuteResponseCommands{
			Status:    "ERROR",
			ErrorCode: "hardError",
		}
	}
	if device == nil {
		return fulfillment.ExecuteResponseCommands{
			Status:    "ERROR",
			ErrorCode: "relinkRequired",
		}
	}

	response, err := f.devmgmt.SendActionCommand(r.r.Context(), *device, execution.Command, execution.Params)
	if err != nil {
		if err == devmgmt.ErrDeviceTimeout {
			return fulfillment.ExecuteResponseCommands{
				Status:    fulfillment.StatusOffline,
				ErrorCode: "offline",
			}
		}
		if err == devmgmt.ErrInvalidSignature {
			return fulfillment.ExecuteResponseCommands{
				Status:    fulfillment.StatusError,
				ErrorCode: "transientError",
			}
		}
		return fulfillment.ExecuteResponseCommands{
			Status:    fulfillment.StatusError,
			ErrorCode: "hardError",
		}
	}

	return fulfillment.ExecuteResponseCommands{
		Status:    response.Status,
		States:    response.State,
		ErrorCode: response.Error,
	}
}

// OnExecute https://developers.google.com/assistant/smarthome/reference/intent/execute
func (f *Fulfillment) onExecuteIntent(r intentRequest) interface{} {
	var executeRequest fulfillment.ExecuteRequest

	if err := json.NewDecoder(r.r.Body).Decode(&executeRequest); err != nil {
		return fulfillment.ExecuteResponse{
			RequestID: r.base.RequestID,
			Payload: fulfillment.ExecuteResponsePayload{
				ErrorCode:   "invalid_payload",
				DebugString: err.Error(),
			},
		}
	}

	var responseCommands []fulfillment.ExecuteResponseCommands
	for _, command := range executeRequest.Inputs[0].Payload.Commands {
		for _, execution := range command.Execution {
			for _, device := range command.Devices {
				exec := f.executeCommand(r, device, execution)
				exec.IDs = []string{device.ID}
				responseCommands = append(responseCommands, exec)
			}
		}
	}

	return fulfillment.ExecuteResponse{
		RequestID: r.base.RequestID,
		Payload: fulfillment.ExecuteResponsePayload{
			Commands: responseCommands,
		},
	}
}
