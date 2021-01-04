package fulfillment

// ---------- Request ----------

// SyncRequestInput ...
type SyncRequestInput struct {
	// Intent request type.
	//
	// (Constant value: "action.devices.SYNC")
	Intent string `json:"intent" bindng:"required,eq=action.devices.SYNC"`
}

// SyncRequest ...
type SyncRequest struct {
	// ID of the request.
	RequestID string `json:"requestId" bindng:"required"`

	// List of inputs matching the intent request.
	Inputs []SyncRequestInput `json:"inputs" bindng:"required"`
}

// ---------- Response ----------

// SyncResponsePayload ...
type SyncResponsePayload struct {
	// Reflects the unique (and immutable) user ID on the agent's platform. The string is opaque to Google, so if there's an immutable form vs a mutable form on the agent side, use the immutable form (e.g. an account number rather than email).
	AgentUserID string `json:"agentUserId" bindng:"required"`

	// List of devices owned by the user. Zero or more devices are returned (zero devices meaning the user has no devices, or has disconnected them all).
	Devices []Device `json:"devices" bindng:"required"`

	// For systematic errors on SYNC
	ErrorCode string `json:"errorCode,omitempty"`

	// Detailed error which will never be presented to users but may be logged or used during development.
	DebugString string `json:"debugString,omitempty"`
}

// SyncResponse ...
type SyncResponse struct {
	// ID of the corresponding request.
	RequestID string `json:"requestId" bindng:"required"`

	// Intent response payload.
	Payload SyncResponsePayload `json:"payload" bindng:"required"`
}

// --------------------
