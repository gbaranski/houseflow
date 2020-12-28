package server

import (
	"net/http"

	"github.com/gbaranski/houseflow/actions/fulfillment"
	"github.com/gbaranski/houseflow/actions/token"
	utils "github.com/gbaranski/houseflow/actions/utils"
	"github.com/gin-gonic/gin"
	"github.com/gin-gonic/gin/binding"
)

func (s *Server) onFulfillment(c *gin.Context) {
	strtoken := utils.ExtractAuthorizationToken(c.Request)
	td, err := token.VerifyToken(strtoken, token.JWTAccessKey)
	if err != nil {
		c.JSON(http.StatusUnauthorized, gin.H{
			"error": err.Error(),
		})
		return
	}

	userID, err := s.db.Redis.FetchToken(td.Claims)
	if err != nil {
		c.JSON(http.StatusUnauthorized, gin.H{
			"error": err.Error(),
		})
		return
	}

	var request fulfillment.SyncRequest
	err = c.MustBindWith(&request, binding.JSON)
	if err != nil {
		c.JSON(http.StatusUnprocessableEntity, gin.H{
			"error": err.Error(),
		})
		return
	}

	var devices []fulfillment.Device

	devices = append(devices, fulfillment.Device{
		ID:   "12345",
		Type: "action.devices.types.WATERHEATER",
		Traits: []string{
			"action.devices.traits.OnOff",
		},
		Name: fulfillment.DeviceName{
			Name: "Simple water heater",
		},
		WillReportState: true,
		Attributes: fulfillment.DeviceAttributes{
			TemperatureUnitForUX: "C",
		},
		DeviceInfo: fulfillment.DeviceInfo{
			Manufacturer: "houseflow-inc",
			Model:        "houseflow-1234",
			HwVersion:    "2.1",
			SwVersion:    "3.0",
		},
	})

	c.JSON(http.StatusOK, fulfillment.SyncResponse{
		RequestID: request.RequestID,
		Payload: fulfillment.SyncPayload{
			AgentUserID: *userID,
			Devices:     devices,
		},
	})

}
