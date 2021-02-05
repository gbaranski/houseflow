package fulfillment

import (
	"context"
	"encoding/json"

	"github.com/gbaranski/houseflow/pkg/devmgmt"
	"github.com/gbaranski/houseflow/pkg/fulfillment"
	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/sirupsen/logrus"
)

func (f *Fulfillment) queryState(ctx context.Context, user types.User, deviceID string) (devices map[string]interface{}) {
	device, err := f.db.GetDeviceByID(ctx, deviceID)
	if err != nil {
		logrus.Errorf("fail retrieve device %s", err.Error())
		return map[string]interface{}{
			"status":           fulfillment.StatusError,
			"errorCode":        "hardError",
			"errorDescription": err.Error(),
		}
	}
	if device == nil {
		return map[string]interface{}{
			"status":    fulfillment.StatusError,
			"errorCode": "deviceNotFound",
		}
	}

	perm, err := f.db.GetUserDevicePermissions(ctx, user.ID, deviceID)
	if err != nil {
		logrus.Errorf("fail retrieve device permission%s", err.Error())
		return map[string]interface{}{
			"status":    fulfillment.StatusError,
			"errorCode": "hardError",
		}
	}
	if !perm.Read {
		logrus.Errorf("user doesn't have read permission to device")
		return map[string]interface{}{
			"status":    fulfillment.StatusError,
			"errorCode": "authFailure",
		}
	}

	res, err := f.devmgmt.FetchDeviceState(ctx, *device)
	if err != nil {
		if err == devmgmt.ErrDeviceTimeout {
			return map[string]interface{}{
				"status": fulfillment.StatusOffline,
			}
		}
		if err == devmgmt.ErrInvalidSignature {
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
func (f *Fulfillment) onQueryIntent(r intentRequest) interface{} {
	var queryRequest fulfillment.QueryRequest
	if err := json.NewDecoder(r.r.Body).Decode(&queryRequest); err != nil {
		return fulfillment.QueryResponse{
			RequestID: r.base.RequestID,
			Payload: fulfillment.QueryResponsePayload{
				ErrorCode:   "invalid_payload",
				DebugString: err.Error(),
			},
		}
	}

	payloadDevices := make(map[string]interface{})
	// Fix it later with waitgroups and goroutines
	for _, device := range queryRequest.Inputs[0].Payload.Devices {
		res := f.queryState(r.r.Context(), r.user, device.ID)
		payloadDevices[device.ID] = res
	}

	return fulfillment.QueryResponse{
		RequestID: r.base.RequestID,
		Payload: fulfillment.QueryResponsePayload{
			Devices: payloadDevices,
		},
	}
}
