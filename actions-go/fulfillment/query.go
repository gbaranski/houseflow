package fulfillment

// ---------- Request ----------

// QueryRequestPayloadDevice device targets to query.
type QueryRequestPayloadDevice struct {
	// Device ID, as per the ID provided in SYNC.
	ID string `json:"id" bindng:"required"`

	// If the opaque customData object is provided in SYNC, it's sent here.
	CustomData map[string]interface{} `json:"customData,omitempty"`
}

// QueryRequestPayload ...
type QueryRequestPayload struct {
	// List of target devices.
	Devices []QueryRequestPayloadDevice `json:"devices" bindng:"required"`
}

// QueryRequestInput type and payload associated with the intent request.
type QueryRequestInput struct {
	// Intent request type
	//
	// (Constant value: "action.devices.QUERY")
	Intent string `json:"intent" bindng:"required,eq=action.devices.QUERY"`

	// QUERY request payload.
	Payload QueryRequestPayload `json:"payload" bindng:"required"`
}

// QueryRequest ...
type QueryRequest struct {
	// ID of the request.
	RequestID string `json:"requestId" bindng:"required"`

	// List of inputs matching the intent request.
	Inputs []QueryRequestInput `json:"inputs" bindng:"required"`
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
	Devices map[string]interface{} `json:"devices" bindng:"required"`
}

// QueryResponse ...
type QueryResponse struct {
	// ID of the corresponding request.
	RequestID string `json:"requestId" bindng:"required"`

	// Intent response payload.
	Payload QueryResponsePayload `json:"payload" bindng:"required"`
}

// --------------------
