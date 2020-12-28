package fulfillment

// QueryDevice for query payload
type QueryDevice struct {
	// Device ID, as per the ID provided in SYNC.
	ID string `json:"id" binding:"required"`
	// If the opaque customData object is provided in SYNC, it's sent here.
	CustomData interface{} `json:"customData"`
}

// QueryPayload request payload.
type QueryPayload struct {
	Devices []QueryDevice `json:"devices" binding:"required"`
}

// QueryInput List of inputs matching the intent request.
type QueryInput struct {
	// Intent request type.
	Intent string `json:"intent" binding:"required"`
	// QUERY request payload.
	Payload QueryPayload `json:"payload" binding:"required"`
}
