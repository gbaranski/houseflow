package server

import (
	"net/http"

	"github.com/gbaranski/houseflow/actions/fulfillment"
	"github.com/gbaranski/houseflow/actions/types"
	"github.com/gin-gonic/gin"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

// OnQuery https://developers.google.com/assistant/smarthome/reference/intent/query
func (s *Server) OnQuery(c *gin.Context, r fulfillment.QueryRequest, user types.User) {
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
	payloadDevices := make(map[string]interface{})
	for _, device := range r.Inputs[0].Payload.Devices {
		// Check if user has proper permission to the specific device
		var correspondingDBDevice *types.Device
		for _, dbDevice := range dbDevices {
			if dbDevice.ID.Hex() == device.ID {
				correspondingDBDevice = &dbDevice
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
