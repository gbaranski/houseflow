package fulfillment

import (
	"github.com/gbaranski/houseflow/pkg/fulfillment"
	"github.com/gbaranski/houseflow/pkg/types"
)

// onSyncIntent handles sync intent https://developers.google.com/assistant/smarthome/reference/intent/sync
func (f *Fulfillment) onSyncIntent(r intentRequest) interface{} {
	devices, err := f.db.GetUserDevices(r.r.Context(), r.user.ID)
	if err != nil {
		return types.ResponseError{
			Name:        "fail_retreive_devices",
			Description: err.Error(),
		}
	}

	resdev := make([]fulfillment.Device, len(devices))
	for i, device := range devices {
		resdev[i] = fulfillment.Device{

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
		}
	}

	return fulfillment.SyncResponse{
		RequestID: r.base.RequestID,
		Payload: fulfillment.SyncResponsePayload{
			AgentUserID: r.user.ID,
			Devices:     resdev,
		},
	}
}
