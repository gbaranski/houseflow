package eventtypes

// WebhookEvent when webhook triggered
type WebhookEvent struct {
	Action   string `json:"action"`
	Username string `json:"username"`
	ClientID string `json:"clientid"`
}

// ConnectedEvent when client connects
type ConnectedEvent struct {
	WebhookEvent
	ProtoVersion       int    `json:"proto_ver"`
	KeepAlive          int    `json:"keepalive"`
	IPAddress          string `json:"ipaddress"`
	ConnectedTimestamp uint64 `json:"connected_at"`
}

// DisconnectedEvent when client disconnected
type DisconnectedEvent struct {
	WebhookEvent
	Reason string `json:"reason"`
}
