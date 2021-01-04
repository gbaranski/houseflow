package fulfillment

// ---------- Request ----------

// QueryRequestPayloadDevice device targets to query.
type QueryRequestPayloadDevice struct {
	// Device ID, as per the ID provided in SYNC.
	ID string `json:"id" binding:"required"`

	// If the opaque customData object is provided in SYNC, it's sent here.
	CustomData map[string]interface{} `json:"customData,omitempty"`
}

// QueryRequestPayload ...
type QueryRequestPayload struct {
	// List of target devices.
	Devices []QueryRequestPayloadDevice `json:"devices" binding:"required"`
}

// QueryRequestInput type and payload associated with the intent request.
type QueryRequestInput struct {
	// Intent request type
	//
	// (Constant value: "action.devices.QUERY")
	Intent string `json:"intent" binding:"required,eq=action.devices.QUERY"`

	// QUERY request payload.
	Payload QueryRequestPayload `json:"payload" binding:"required"`
}

// QueryRequest ...
type QueryRequest struct {
	// ID of the request.
	RequestID string `json:"requestId" binding:"required"`

	// List of inputs matching the intent request.
	Inputs []QueryRequestInput `json:"inputs" binding:"required"`
}

// ---------- Response ----------

// QueryResponsePayload ...
type QueryResponsePayload struct {
	// https://developers.google.com/assistant/smarthome/reference/intent/query#response
	//
	// Must contain
	//
	// online - Indicates if the device is online (that is, reachable) or not.
	//
	// status - Result of the query operation.
	//
	// And other strictly related to specific device data
	Devices map[string]interface{} `json:"devices" binding:"required"`
}

// QueryResponse ...
type QueryResponse struct {
	// ID of the corresponding request.
	RequestID string `json:"requestId" binding:"required"`

	// Intent response payload.
	Payload QueryResponsePayload `json:"payload" binding:"required"`
}

// --------------------
