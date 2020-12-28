package fulfillment

// DeviceAttributes attributes for Device
type DeviceAttributes struct {
	// F/C
	TemperatureUnitForUX string `json:"temperatureUnitForUX"`
}

// DeviceInfo Contains fields describing the device for use in one-off logic if needed (e.g. 'broken firmware version X of light Y requires adjusting color', or 'security flaw requires notifying all users of firmware Z').
type DeviceInfo struct {
	// Especially useful when the developer is a hub for other devices. Google may provide a standard list of manufacturers here so that e.g. TP-Link and Smartthings both describe 'osram' the same way.
	Manufacturer string `json:"manufacturer"`
	// The model or SKU identifier of the particular device.
	Model string `json:"model"`
	// Specific version number attached to the hardware if available.
	HwVersion string `json:"hwVersion"`
	// Specific version number attached to the software/firmware, if available.
	SwVersion string `json:"swVersion"`
}

// DeviceName names of this device.
type DeviceName struct {
	// List of names provided by the developer rather than the user, often manufacturer names, SKUs, etc.
	DefaultNames []string `json:"defaultNames"`
	// Primary name of the device, generally provided by the user. This is also the name the Assistant will prefer to describe the device in responses.
	Name string `json:"name" binding:"required"`
	// Additional names provided by the user for the device.
	Nicknames []string `json:"nicknames"`
}

// Device ...
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
	RoomHint string `json:"roomHint"`
	// Contains fields describing the device for use in one-off logic if needed (e.g. 'broken firmware version X of light Y requires adjusting color', or 'security flaw requires notifying all users of firmware Z').
	DeviceInfo DeviceInfo `json:"deviceInfo"`
	// Aligned with per-trait attributes described in each trait schema reference.
	Attributes DeviceAttributes `json:"attributes"`
	// This is a special object defined by the developer which will be attached to future QUERY and EXECUTE requests. Developers can use this object to store additional information about the device to improve performance or routing within their cloud, such as the global region of the device. Data in this object has a few constraints: No Personally Identifiable Information. Data should change rarely, akin to other attributes -- so this should not contain real-time state. The total object is limited to 512 bytes per device.
	CustomData interface{} `json:"customData"`
	// List of alternate IDs used to identify a cloud synced device for local execution.
	OtherDeviceIds []struct {
		// The agent's ID. Generally, this is the project ID in the Actions console.
		AgentID string `json:"agentID"`
		// Device ID defined by the agent. The device ID must be unique.
		DeviceID string `json:"deviceId" binding:"required"`
	} `json:"otherDeviceIds"`
}
