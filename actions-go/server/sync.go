package server

import (
	"net/http"

	"github.com/gbaranski/houseflow/actions/fulfillment"
	"github.com/gbaranski/houseflow/actions/types"
	"github.com/gin-gonic/gin"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

// OnSync handles sync intent https://developers.google.com/assistant/smarthome/reference/intent/sync
func (s *Server) onSync(c *gin.Context, r fulfillment.SyncRequest, user types.User) {
	var deviceIDs []primitive.ObjectID
	for _, id := range user.Devices {
		objID, err := primitive.ObjectIDFromHex(id)
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{
				"error":             "convert_object_id_fail",
				"error_description": err.Error(),
			})
			return
		}
		deviceIDs = append(deviceIDs, objID)
	}
	dbDevices, err := s.db.Mongo.GetUserDevices(deviceIDs)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"error":             "get_devices_fail",
			"error_description": err.Error(),
		})
		return
	}

	var fdevices []fulfillment.Device
	for _, device := range dbDevices {
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
