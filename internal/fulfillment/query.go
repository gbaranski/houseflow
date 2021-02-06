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
		logrus.Errorf("fail retrieving device ID: %s from database %s\n", deviceID, err.Error())
		return map[string]interface{}{
			"status":           fulfillment.StatusError,
			"errorCode":        "hardError",
			"errorDescription": err.Error(),
		}
	}
	if device == nil {
		logrus.Errorf("device ID: %s not found in database\n", deviceID)
		return map[string]interface{}{
			"status":    fulfillment.StatusError,
			"errorCode": "deviceNotFound",
		}
	}

	perm, err := f.db.GetUserDevicePermissions(ctx, user.ID, deviceID)
	if err != nil {
		logrus.Errorf("fail retrieve device permission for Device ID: %s from UserID: %s, err: %s\n", deviceID, user.ID, err.Error())
		return map[string]interface{}{
			"status":    fulfillment.StatusError,
			"errorCode": "hardError",
		}
	}
	if !perm.Read {
		logrus.Errorf("user ID: %s doesnt have read permissions to device ID: %s", user.ID, deviceID)
		return map[string]interface{}{
			"status":    fulfillment.StatusError,
			"errorCode": "authFailure",
		}
	}

	res, err := f.devmgmt.FetchDeviceState(ctx, *device)
	if err != nil {
		if err == devmgmt.ErrDeviceTimeout {
			logrus.Errorf("device ID: %s timed out\n", device.ID)
			return map[string]interface{}{
				"status": fulfillment.StatusOffline,
			}
		}
		if err == devmgmt.ErrInvalidSignature {
			logrus.Errorf("device ID: %s returned invalid signature\n", device.ID)
			return map[string]interface{}{
				"status":    fulfillment.StatusError,
				"errorCode": "transientError",
			}
		}
		logrus.Errorf("device ID: %s returned unrecognized error: %s\n", device.ID, err.Error())
		return map[string]interface{}{
			"status":    fulfillment.StatusError,
			"errorCode": "hardError",
		}
	}
	mapResponse := map[string]interface{}{
		"status":    res.Status,
		"errorCode": res.Error,
	}
	for k, v := range res.State {
		mapResponse[k] = v
	}
	logrus.Infof("device ID: %s responsed to Query request with Status: %s and State: %+v\n", device.ID, res.Status, res.State)
	return mapResponse

}

// OnQuery https://developers.google.com/assistant/smarthome/reference/intent/query
func (f *Fulfillment) onQueryIntent(r intentRequest) interface{} {
	logrus.Infof("Received Query intent from User ID:%s\n", r.user.ID)
	var queryRequest fulfillment.QueryRequest
	if err := json.NewDecoder(r.r.Body).Decode(&queryRequest); err != nil {
		logrus.Errorf("fail decoding query request body err: %s\n", err.Error())
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
