package fulfillment

import (
	"encoding/json"

	"github.com/gbaranski/houseflow/pkg/devmgmt"
	"github.com/gbaranski/houseflow/pkg/fulfillment"
	"github.com/sirupsen/logrus"
)

func (f *Fulfillment) executeCommand(
	r intentRequest,
	targetDevice fulfillment.QueryRequestPayloadDevice,
	execution fulfillment.ExecuteRequestExecution,
) fulfillment.ExecuteResponseCommands {

	perms, err := f.db.GetUserDevicePermissions(r.r.Context(), r.user.ID, targetDevice.ID)
	if err != nil {
		logrus.Errorf("fail retrieve device ID: %s permissions for user ID: %s, error: %s \n", r.user.ID, targetDevice.ID, err.Error())
		return fulfillment.ExecuteResponseCommands{
			Status:    "ERROR",
			ErrorCode: "hardError",
		}
	}
	if !perms.Execute {
		logrus.Errorf("user ID: %s does not have execute perm to DeviceID: %s\n", r.user.ID, targetDevice.ID)
		return fulfillment.ExecuteResponseCommands{
			Status:    fulfillment.StatusError,
			ErrorCode: "authFailure",
		}
	}
	device, err := f.db.GetDeviceByID(r.r.Context(), targetDevice.ID)
	if err != nil {
		logrus.Errorf("fail retrieve device ID: %s, error: %s\n", targetDevice.ID, err.Error())
		return fulfillment.ExecuteResponseCommands{
			Status:    "ERROR",
			ErrorCode: "hardError",
		}
	}
	if device == nil {
		logrus.Errorf("device ID: %s not found in database %s\n", targetDevice.ID, err.Error())
		return fulfillment.ExecuteResponseCommands{
			Status:    "ERROR",
			ErrorCode: "relinkRequired",
		}
	}

	response, err := f.devmgmt.SendActionCommand(r.r.Context(), *device, execution.Command, execution.Params)
	if err != nil {
		if err == devmgmt.ErrDeviceTimeout {
			logrus.Errorf("device ID: %s timed out\n", device.ID)
			return fulfillment.ExecuteResponseCommands{
				Status:    fulfillment.StatusOffline,
				ErrorCode: "offline",
			}
		}
		if err == devmgmt.ErrInvalidSignature {
			logrus.Errorf("device ID: %s returned invalid signature\n", device.ID)
			return fulfillment.ExecuteResponseCommands{
				Status:    fulfillment.StatusError,
				ErrorCode: "transientError",
			}
		}
		logrus.Errorf("device ID: %s returned unrecognized error: %s\n", device.ID, err.Error())
		return fulfillment.ExecuteResponseCommands{
			Status:    fulfillment.StatusError,
			ErrorCode: "hardError",
		}
	}

	logrus.Infof("device ID: %s responsed to Execute request with Status: %s and State: %+v\n", device.ID, response.Status, response.State)
	return fulfillment.ExecuteResponseCommands{
		Status:    response.Status,
		States:    response.State,
		ErrorCode: response.Error,
	}
}

// OnExecute https://developers.google.com/assistant/smarthome/reference/intent/execute
func (f *Fulfillment) onExecuteIntent(r intentRequest) interface{} {
	logrus.Infof("Received Execute intent from User ID:%s\n", r.user.ID)
	var executeRequest fulfillment.ExecuteRequest

	if err := json.NewDecoder(r.r.Body).Decode(&executeRequest); err != nil {
		logrus.Errorf("fail decoding execute request %s\n", err.Error())
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
