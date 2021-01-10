package server

import (
	"context"
	"fmt"
	"net/http"
	"time"

	"github.com/gbaranski/houseflow/pkg/fulfillment"
	"github.com/gbaranski/houseflow/pkg/mqtt"
	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/gbaranski/houseflow/pkg/utils"
	"github.com/gin-gonic/gin"
)

// OnExecute https://developers.google.com/assistant/smarthome/reference/intent/execute
func (s *Server) onExecute(c *gin.Context, r fulfillment.ExecuteRequest, user types.User, userDevices []types.Device) {
	var responseCommands []fulfillment.ExecuteResponseCommands
	for _, command := range r.Inputs[0].Payload.Commands {
		for _, execution := range command.Execution {
			for _, device := range command.Devices {
        // Search userDevices for the devices which Execute request wants to match
				var correspondingDBDevice *types.Device
				for _, userDevice := range userDevices {
					if userDevice.ID.Hex() == device.ID {
						correspondingDBDevice = &userDevice
						break
					}
				}

				if correspondingDBDevice == nil {
					responseCommands = append(
						responseCommands,
						fulfillment.ExecuteResponseCommands{
							IDs:       []string{device.ID},
							Status:    fulfillment.StatusError,
							ErrorCode: "relinkRequired",
						})
					continue
				}

				request := types.DeviceRequest{
					CorrelationData: utils.GenerateRandomString(16),
					State:           execution.Params,
					Command:         execution.Command,
				}
				ctx, cancel := context.WithTimeout(context.Background(), time.Second*3)
				defer cancel()
				deviceResponse, err := s.mqtt.SendRequestWithResponse(ctx, *correspondingDBDevice, request)
				if err != nil {
					fmt.Println("Error occured when executing on device: ", err.Error())
					if err == mqtt.ErrDeviceTimeout {
						responseCommands = append(responseCommands, fulfillment.ExecuteResponseCommands{
							IDs:       []string{device.ID},
							Status:    fulfillment.StatusOffline,
							ErrorCode: "offline",
							States: map[string]interface{}{
								"online": false,
							},
						})
						// should also change state in db
					} else if err == mqtt.ErrInvalidSignature {
						responseCommands = append(responseCommands, fulfillment.ExecuteResponseCommands{
							IDs:       []string{device.ID},
							Status:    fulfillment.StatusError,
							ErrorCode: "transientError",
							States:    correspondingDBDevice.State,
						})
					} else {
						fmt.Println("Unknown error, ", err.Error())
						responseCommands = append(responseCommands, fulfillment.ExecuteResponseCommands{
							IDs:       []string{device.ID},
							Status:    fulfillment.StatusError,
							ErrorCode: "hardError",
							States:    correspondingDBDevice.State,
						})
					}
					continue
				}
				responseCommands = append(responseCommands, fulfillment.ExecuteResponseCommands{
					IDs:    []string{device.ID},
					Status: "SUCCESS",
					States: deviceResponse.State,
				})
				ctx, cancel = context.WithTimeout(context.Background(), time.Second*3)
				defer cancel()
				err = s.mongo.UpdateDeviceState(ctx, correspondingDBDevice.ID, deviceResponse.State)
				if err != nil {
					fmt.Println("failed updating state, ", err.Error())
				}

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
