package fulfillment

// Input of request
type Input struct {
	// Intent request type.
	Intent string `json:"intent" binding:"required"`
	// Request payload, only for Query/Execute
	Payload []interface{} `json:"payload"`
}

// Request for fulfillment
type Request struct {
	// ID of the request.
	RequestID string  `json:"requestId" binding:"required"`
	Inputs    []Input `json:"inputs" binding:"required"`
}
