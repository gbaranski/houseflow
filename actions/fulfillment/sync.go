package fulfillment

// SyncInput of the fulfillmentRequest
type SyncInput struct {
	Intent string `json:"intent" binding:"required"`
}

// SyncPayload payload for Response
type SyncPayload struct {
	// Reflects the unique (and immutable) user ID on the agent's platform. The string is opaque to Google, so if there's an immutable form vs a mutable form on the agent side, use the immutable form (e.g. an account number rather than email).
	AgentUserID string `json:"agentUserId" binding:"required"`
	// For systematic errors on SYNC
	ErrorCode string `json:"errorCode"`
	// Detailed error which will never be presented to users but may be logged or used during development.
	DebugString string `json:"debugString"`
	// List of devices owned by the user. Zero or more devices are returned (zero devices meaning the user has no devices, or has disconnected them all).
	Devices []Device `json:"devices" binding:"required"`
}

// SyncResponse response for fulfillment request
type SyncResponse struct {
	// ID of the corresponding request.
	RequestID string `json:"requestId" binding:"required"`
	// Intent response payload.
	Payload SyncPayload `json:"payload" binding:"required"`
}

// SyncRequest is request type
type SyncRequest struct {
	RequestID string      `json:"requestId" binding:"required"`
	Inputs    []SyncInput `json:"inputs" binding:"required"`
}
