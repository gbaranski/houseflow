package fulfillment

import (
	"net/http"

	"github.com/gbaranski/houseflow/pkg/fulfillment"
	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/gin-gonic/gin"
)

// OnSync handles sync intent https://developers.google.com/assistant/smarthome/reference/intent/sync
func (f *Fulfillment) onSync(c *gin.Context, r fulfillment.SyncRequest, user types.User, userDevices []types.Device) {
	var fdevices []fulfillment.Device
	for _, device := range userDevices {
		fdevices = append(fdevices, fulfillment.Device{
			ID:              device.ID.Hex(),
			Type:            device.Type,
			Traits:          device.Traits,
			Name:            device.Name,
			WillReportState: device.WillReportState,
			RoomHint:        device.RoomHint,
			DeviceInfo:      device.DeviceInfo,
			// Attributes:      device.Attributes, // unused atm
			// CustomData:      device.CustomData, // unused atm
			// OtherDevicesIDs: device.OtherDevicesIDs, // unused atm
		})
	}

	response := fulfillment.SyncResponse{
		RequestID: r.RequestID,
		Payload: fulfillment.SyncResponsePayload{
			AgentUserID: user.ID.Hex(),
			Devices:     fdevices,
		},
	}
	c.JSON(http.StatusOK, response)
}
