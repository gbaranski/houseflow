package fulfillment

import (
	"github.com/gbaranski/houseflow/pkg/fulfillment"
	"github.com/gbaranski/houseflow/pkg/types"
	"github.com/sirupsen/logrus"
)

// onSyncIntent handles sync intent https://developers.google.com/assistant/smarthome/reference/intent/sync
func (f *Fulfillment) onSyncIntent(r intentRequest) interface{} {
	logrus.Infof("Received Sync intent from User ID:%s\n", r.user.ID)
	devices, err := f.db.GetUserDevices(r.r.Context(), r.user.ID)
	if err != nil {
		logrus.Errorf("Fail retrieving user devices:%s\n", err.Error())
		return types.ResponseError{
			Name:        "fail_retrieve_devices",
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
