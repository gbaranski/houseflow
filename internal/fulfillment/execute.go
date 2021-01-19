package fulfillment

import (
	"context"
	"net/http"

	"github.com/gbaranski/houseflow/pkg/fulfillment"
	"github.com/gbaranski/houseflow/pkg/mqtt"
	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/gbaranski/houseflow/pkg/utils"
	"github.com/gin-gonic/gin"
)

func (f *Fulfillment) executeCommand(
	ctx context.Context,
	user types.User,
	targetDevice fulfillment.QueryRequestPayloadDevice,
	execution fulfillment.ExecuteRequestExecution,
) fulfillment.ExecuteResponseCommands {

	perms, err := f.db.GetUserDevicePermissions(ctx, user.ID, targetDevice.ID)
	if err != nil {
		return fulfillment.ExecuteResponseCommands{
			IDs:       []string{targetDevice.ID},
			Status:    "ERROR",
			ErrorCode: "hardError",
		}
	}
	if !perms.Execute {
		return fulfillment.ExecuteResponseCommands{
			IDs:       []string{targetDevice.ID},
			Status:    fulfillment.StatusError,
			ErrorCode: "authFailure",
		}
	}
	device, err := f.db.GetDeviceByID(ctx, targetDevice.ID)
	if err != nil {
		return fulfillment.ExecuteResponseCommands{
			IDs:       []string{targetDevice.ID},
			Status:    "ERROR",
			ErrorCode: "hardError",
		}
	}

	req := types.DeviceRequest{
		CorrelationData: utils.GenerateRandomString(16),
		State:           execution.Params,
		Command:         execution.Command,
	}
	response, err := f.dm.SendRequestWithResponse(ctx, device, req)
	if err != nil {
		if err == mqtt.ErrDeviceTimeout {
			return fulfillment.ExecuteResponseCommands{
				IDs:       []string{targetDevice.ID},
				Status:    fulfillment.StatusOffline,
				ErrorCode: "offline",
			}
		}
		if err == mqtt.ErrInvalidSignature {
			return fulfillment.ExecuteResponseCommands{
				IDs:       []string{targetDevice.ID},
				Status:    fulfillment.StatusError,
				ErrorCode: "transientError",
			}
		}
		return fulfillment.ExecuteResponseCommands{
			IDs:       []string{targetDevice.ID},
			Status:    fulfillment.StatusError,
			ErrorCode: "hardError",
		}
	}

	return fulfillment.ExecuteResponseCommands{
		IDs:    []string{targetDevice.ID},
		Status: "SUCCESS",
		States: response.State,
	}
}

// OnExecute https://developers.google.com/assistant/smarthome/reference/intent/execute
func (f *Fulfillment) onExecute(c *gin.Context, r fulfillment.ExecuteRequest, user types.User) {
	var responseCommands []fulfillment.ExecuteResponseCommands
	for _, command := range r.Inputs[0].Payload.Commands {
		for _, execution := range command.Execution {
			for _, device := range command.Devices {
				exec := f.executeCommand(c.Request.Context(), user, device, execution)
				responseCommands = append(responseCommands, exec)
			}
		}
	}

	response := fulfillment.ExecuteResponse{
		RequestID: r.RequestID,
		Payload: fulfillment.ExecuteResponsePayload{
			Commands: responseCommands,
		},
	}
	c.JSON(http.StatusOK, response)
}
