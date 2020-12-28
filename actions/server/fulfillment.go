package server

import (
	"net/http"

	"github.com/gbaranski/houseflow/actions/token"
	utils "github.com/gbaranski/houseflow/actions/utils"
	"github.com/gin-gonic/gin"
	"github.com/gin-gonic/gin/binding"
)

// Input of the fulfillmentRequest
type Input struct {
	Intent string `json:"intent" binding:"required"`
}

// FulfillmentRequest is request type
type FulfillmentRequest struct {
	RequestID string  `json:"requestId" binding:"required"`
	Inputs    []Input `json:"inputs" binding:"required"`
}

// DeviceName name for Device
type DeviceName struct {
	Name string `json:"name" binding:"required"`
}

// DeviceAttributes attributes for Device
type DeviceAttributes struct {
	// F/C
	TemperatureUnitForUX string `json:"temperatureUnitForUX" binding:"required"`
}

// DeviceInfo information about Device
type DeviceInfo struct {
	Manufacturer string `json:"manufacturer" binding:"required"`
	Model        string `json:"model" binding:"required"`
	HwVersion    string `json:"hwVersion" binding:"required"`
	SwVersion    string `json:"swVersion" binding:"required"`
}

// Device ...
type Device struct {
	ID              string           `json:"id" binding:"required"`
	Type            string           `json:"type" binding:"required"`
	Traits          []string         `json:"traits" binding:"required"`
	Name            DeviceName       `json:"name" binding:"required"`
	WillReportState bool             `json:"willReportState" binding:"required"`
	Attributes      DeviceAttributes `json:"attributes" binding:"required"`
	DeviceInfo      DeviceInfo       `json:"deviceInfo" binding:"required"`
}

// FulfillmentPayload payload for FulfillmentResponse
type FulfillmentPayload struct {
	AgentUserID string   `json:"agentUserId" binding:"required"`
	Devices     []Device `json:"devices" binding:"required"`
}

// FulfillmentResponse response for fulfillment request
type FulfillmentResponse struct {
	RequestID string             `json:"requestId" binding:"required"`
	Payload   FulfillmentPayload `json:"payload" binding:"required"`
}

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

	var request FulfillmentRequest
	err = c.MustBindWith(&request, binding.JSON)
	if err != nil {
		c.JSON(http.StatusUnprocessableEntity, gin.H{
			"error": err.Error(),
		})
		return
	}

	var devices []Device

	devices = append(devices, Device{
		ID:   "12345",
		Type: "action.devices.types.WATERHEATER",
		Traits: []string{
			"action.devices.traits.OnOff",
		},
		Name: DeviceName{
			Name: "Simple water heater",
		},
		WillReportState: true,
		Attributes: DeviceAttributes{
			TemperatureUnitForUX: "C",
		},
		DeviceInfo: DeviceInfo{
			Manufacturer: "houseflow-inc",
			Model:        "houseflow-1234",
			HwVersion:    "2.1",
			SwVersion:    "3.0",
		},
	})

	c.JSON(http.StatusOK, FulfillmentResponse{
		RequestID: request.RequestID,
		Payload: FulfillmentPayload{
			AgentUserID: *userID,
			Devices:     devices,
		},
	})

}
