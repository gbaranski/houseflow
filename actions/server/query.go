package server

import (
	"net/http"

	"github.com/gbaranski/houseflow/pkg/fulfillment"
	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/gin-gonic/gin"
)

// OnQuery https://developers.google.com/assistant/smarthome/reference/intent/query
func (s *Server) OnQuery(c *gin.Context, r fulfillment.QueryRequest, user types.User, userDevices []types.Device) {
	payloadDevices := make(map[string]interface{})
	for _, device := range r.Inputs[0].Payload.Devices {
		// Check if user has proper permission to the specific device
		var correspondingDBDevice *types.Device
		for _, userDevice := range userDevices {
			if userDevice.ID.Hex() == device.ID {
				correspondingDBDevice = &userDevice
				break
			}
		}
		if correspondingDBDevice == nil {
			payloadDevices[device.ID] = gin.H{
				"status":    fulfillment.StatusError,
				"errorCode": "relinkRequired",
			}
			continue
		}

		payloadDevice := gin.H{
			"status": fulfillment.StatusSuccess,
		}
		for k, v := range correspondingDBDevice.State {
			payloadDevice[k] = v
		}
		payloadDevices[device.ID] = payloadDevice
	}

	response := fulfillment.QueryResponse{
		RequestID: r.RequestID,
		Payload: fulfillment.QueryResponsePayload{
			Devices: payloadDevices,
		},
	}
	c.JSON(http.StatusOK, response)
}
