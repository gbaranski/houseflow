package fulfillment

import (
	"context"
	"net/http"

	"github.com/gbaranski/houseflow/pkg/fulfillment"
	"github.com/gbaranski/houseflow/pkg/mqtt"
	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/gin-gonic/gin"
)

func (f *Fulfillment) queryState(ctx context.Context, user types.User, deviceID string) (devices map[string]interface{}) {
	perm, err := f.db.GetUserDevicePermissions(ctx, user.ID, deviceID)
	if err != nil {
		return map[string]interface{}{
			"status":    fulfillment.StatusError,
			"errorCode": "hardError",
		}
	}
	if !perm.Read {
		return map[string]interface{}{
			"status":      fulfillment.StatusError,
			"errorCode":   "authFailure",
			"debugString": "missing execute permission",
		}
	}
	res, err := f.dm.FetchDeviceState(ctx, deviceID)
	if err != nil {
		if err == mqtt.ErrDeviceTimeout {
			return map[string]interface{}{
				"status": fulfillment.StatusOffline,
			}
		}
		if err == mqtt.ErrInvalidSignature {
			return map[string]interface{}{
				"status":    fulfillment.StatusError,
				"errorCode": "transientError",
			}
		}
		return map[string]interface{}{
			"status":    fulfillment.StatusError,
			"errorCode": "hardError",
		}
	}
	mapResponse := map[string]interface{}{
		"status": fulfillment.StatusSuccess,
	}
	for k, v := range res.State {
		mapResponse[k] = v
	}
	return mapResponse

}

// OnQuery https://developers.google.com/assistant/smarthome/reference/intent/query
func (f *Fulfillment) onQueryIntent(c *gin.Context, r fulfillment.QueryRequest, user types.User) {
	payloadDevices := make(map[string]interface{})
	// Fix it later with waitgroups and goroutines
	for _, device := range r.Inputs[0].Payload.Devices {
		res := f.queryState(c.Request.Context(), user, device.ID)
		payloadDevices[device.ID] = res
	}

	response := fulfillment.QueryResponse{
		RequestID: r.RequestID,
		Payload: fulfillment.QueryResponsePayload{
			Devices: payloadDevices,
		},
	}
	c.JSON(http.StatusOK, response)
}
