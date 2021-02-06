package fulfillment

const (
	// SyncIntent https://developers.google.com/assistant/smarthome/reference/intent/sync
	SyncIntent = "action.devices.SYNC"
	// QueryIntent https://developers.google.com/assistant/smarthome/reference/intent/query
	QueryIntent = "action.devices.QUERY"
	// ExecuteIntent https://developers.google.com/assistant/smarthome/reference/intent/execute
	ExecuteIntent = "action.devices.EXECUTE"
	// DisconnectIntent https://developers.google.com/assistant/smarthome/reference/intent/disconnect
	DisconnectIntent = "action.devices.DISCONNECT"
)

const (
	// StatusSuccess confirm that the command succeeded.
	StatusSuccess = "SUCCESS"
	// StatusPending command is enqueued but expected to succeed.
	StatusPending = "PENDING"
	// StatusOffline target device is in offline state or unreachable.
	StatusOffline = "OFFLINE"
	// StatusExceptions There is an issue or alert associated with a command. The command could succeed or fail. This status type is typically set when you want to send additional information about another connected device.
	StatusExceptions = "EXCEPTIONS"
	// StatusError Target device is unable to perform the command.
	StatusError = "ERROR"
)

// DeviceInfo contains fields describing the device for use in one-off logic if needed (e.g. 'broken firmware version X of light Y requires adjusting color', or 'security flaw requires notifying all users of firmware Z').
type DeviceInfo struct {
	// Especially useful when the developer is a hub for other devices. Google may provide a standard list of manufacturers here so that e.g. TP-Link and Smartthings both describe 'osram' the same way.
	Manufacturer string `json:"manufacturer,omitempty" bson:"manufacturer,omitempty"`

	// The model or SKU identifier of the particular device.
	Model string `json:"model,omitempty" bson:"model,omitempty"`

	// Specific version number attached to the hardware if available.
	HwVersion string `json:"hwVersion,omitempty" bson:"hwVersion,omitempty"`

	// Specific version number attached to the software/firmware, if available.
	SwVersion string `json:"swVersion,omitempty" bson:"swVersion,omitempty"`
}

// OtherDeviceID Alternate device ID.
type OtherDeviceID struct {
	// The agent's ID. Generally, this is the project ID in the Actions console.
	AgentID string `json:"agentId,omitempty" bson:"agentId,omitempty"`

	// Device ID defined by the agent. The device ID must be unique.
	DeviceID string `json:"deviceId" bson:"deviceId"`
}

// DeviceName names of this device.
type DeviceName struct {
	// Primary name of the device, generally provided by the user. This is also the name the Assistant will prefer to describe the device in responses.
	Name string `json:"name" binding:"required" bson:"name"`

	// List of names provided by the developer rather than the user, often manufacturer names, SKUs, etc.
	DefaultNames []string `json:"defaultNames,omitempty" bson:"defaultNames,omitempty"`

	// Additional names provided by the user for the device.
	Nicknames []string `json:"nicknames,omitempty" bson:"nicknames,omitempty"`
}

// Device metadata.
type Device struct {
	// The ID of the device in the developer's cloud. This must be unique for the user and for the developer, as in cases of sharing we may use this to dedupe multiple views of the same device. It should be immutable for the device; if it changes, the Assistant will treat it as a new device.
	ID string `json:"id" binding:"required"`

	// The hardware type of device.
	Type string `json:"type" binding:"required"`

	// List of traits this device has. This defines the commands, attributes, and states that the device supports.
	Traits []string `json:"traits" binding:"required"`

	// Names of this device.
	Name DeviceName `json:"name" binding:"required"`

	// Indicates whether this device will have its states updated by the Real Time Feed. (true to use the Real Time Feed for reporting state, and false to use the polling model.)
	WillReportState bool `json:"willReportState" binding:"required"`

	// Provides the current room of the device in the user's home to simplify setup.
	RoomHint string `json:"roomHint,omitempty"`

	// Contains fields describing the device for use in one-off logic if needed (e.g. 'broken firmware version X of light Y requires adjusting color', or 'security flaw requires notifying all users of firmware Z').
	DeviceInfo DeviceInfo `json:"deviceInfo"`

	// Aligned with per-trait attributes described in each trait schema reference.
	Attributes map[string]interface{} `json:"attributes,omitempty"`

	// This is a special object defined by the developer which will be attached to future QUERY and EXECUTE requests. Developers can use this object to store additional information about the device to improve performance or routing within their cloud, such as the global region of the device. Data in this object has a few constraints: No Personally Identifiable Information. Data should change rarely, akin to other attributes -- so this should not contain real-time state. The total object is limited to 512 bytes per device.
	CustomData map[string]interface{} `json:"customData,omitempty"`

	// List of alternate IDs used to identify a cloud synced device for local execution.
	OtherDeviceIDs []OtherDeviceID `json:"otherDeviceIds,omitempty"`
}

// BaseRequestInput ...
type BaseRequestInput struct {
	Intent string `json:"intent" binding:"required"`
}

// BaseRequest is common for all intents
type BaseRequest struct {
	// ID of the corresponding request.
	RequestID string `json:"requestId" binding:"required"`

	Inputs []BaseRequestInput `json:"inputs" binding:"required"`
}
