package fulfillment

// ---------- Request ----------

// ExecuteRequestExecution ...
type ExecuteRequestExecution struct {
	// The command to execute, usually with accompanying parameters.
	Command string `json:"command" binding:"required"`

	// Aligned with the parameters for each command.
	Params map[string]interface{} `json:"params,omitempty"`
}

// ExecuteRequestCommands ...
type ExecuteRequestCommands struct {
	// List of target devices.
	Devices []QueryRequestPayloadDevice `json:"devices" binding:"required"`

	// List of commands to execute on target devices.
	Execution []ExecuteRequestExecution `json:"execution" binding:"required"`
}

// ExecuteRequestPayload ...
type ExecuteRequestPayload struct {
	// List of device target and command pairs.
	Commands []ExecuteRequestCommands `json:"commands" binding:"required"`
}

// ExecuteRequestInput type and payload associated with the intent request.
type ExecuteRequestInput struct {
	// Intent request type.
	//
	// (Constant value: "action.devices.EXECUTE")
	Intent string `json:"intent" binding:"required,eq=action.devices.EXECUTE"`

	// EXECUTE request payload.
	Payload ExecuteRequestPayload `json:"payload" binding:"required"`
}

// ExecuteRequest ...
type ExecuteRequest struct {
	// ID of the request.
	RequestID string `json:"requestId" binding:"required"`

	//List of inputs matching the intent request.
	Inputs []ExecuteRequestInput `json:"inputs" binding:"required"`
}

// ---------- Response ----------

// ExecuteResponseCommands ...
type ExecuteResponseCommands struct {
	// List of device IDs corresponding to this status.
	IDs []string `json:"ids" binding:"required"`

	// Result of the execute operation, must be one of Status...
	Status string `json:"status" binding:"required,oneof=SUCCESS PENDING OFFLINE EXCEPTIONS ERROR"`

	// Aligned with per-trait states described in each trait schema reference. These are the states after execution, if available.
	States map[string]interface{} `json:"debugString,omitempty"`

	// Expanding ERROR state if needed from the preset error codes, which will map to the errors presented to users.
	ErrorCode string `json:"errorCode,omitempty"`
}

// ExecuteResponsePayload ...
type ExecuteResponsePayload struct {
	// An error code for the entire transaction for auth failures and developer system unavailability. For individual device errors, use the errorCode within the device object.
	ErrorCode string `json:"errorCode,omitempty"`

	// Detailed error which will never be presented to users but may be logged or used during development.
	DebugString string `json:"debugString,omitempty"`

	// Each object contains one or more devices with response details. N.B. These may not be grouped the same way as in the request. For example, the request might turn 7 lights on, with 3 lights succeeding and 4 failing, thus with two groups in the response.
	Commands []ExecuteResponseCommands `json:"commands" binding:"required"`
}

// ExecuteResponse ...
type ExecuteResponse struct {
	// ID of the corresponding request.
	RequestID string `json:"requestId" binding:"required"`

	// Intent response payload.
	Payload ExecuteResponsePayload `json:"payload" binding:"required"`
}
