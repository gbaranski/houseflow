package fulfillment

import (
	"net/http"

	"github.com/gbaranski/houseflow/pkg/fulfillment"
	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/gin-gonic/gin"
)

// OnSync handles sync intent https://developers.google.com/assistant/smarthome/reference/intent/sync
func (f *Fulfillment) onSyncIntent(c *gin.Context, r fulfillment.SyncRequest, user types.User) {
	devices, err := f.db.GetUserDevices(c.Request.Context(), user.ID)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"error":             "fail_retreive_devices",
			"error_description": err.Error(),
		})
		return
	}

	var fdevices []fulfillment.Device
	for _, device := range devices {
		fdevices = append(fdevices, fulfillment.Device{
			ID:              device.ID,
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
			AgentUserID: user.ID,
			Devices:     fdevices,
		},
	}
	c.JSON(http.StatusOK, response)
}
