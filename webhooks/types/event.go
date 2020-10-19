package eventtypes

// WebhookEvent when webhook triggered
type WebhookEvent struct {
	Action string `json:"action"`
}

// ConnectedEvent when client connects
type ConnectedEvent struct {
	WebhookEvent
	Username           string `json:"username"`
	ClientID           string `json:"clientid"`
	ProtoVersion       int    `json:"proto_ver"`
	KeepAlive          int    `json:"keepalive"`
	IPAddress          string `json:"ipaddress"`
	ConnectedTimestamp uint64 `json:"connected_at"`
}

// DisconnectedEvent when client disconnected
type DisconnectedEvent struct {
	WebhookEvent
	Username string `json:"username"`
	ClientID string `json:"clientid"`
	Reason   string `json:"reason"`
}

// MessageEvent when client sends message
type MessageEvent struct {
	WebhookEvent
	ToClientID   string `json:"clientid"`
	FromClientID string `json:"from_client_id"`
	FromUsername string `json:"from_username"`
	Topic        string `json:"topic"`
	Qos          int    `json:"qos"`
	Retain       bool   `json:"retain"`
	Payload      string `json:"payload"`
	Timestamp    int    `json:"ts"`
}

// DeviceRequest refers to database device history
type DeviceRequest struct {
	Request   string
	Username  string
	IPAddress string
	Timestamp int
}

// MqttClientData Clent data received from API
type MqttClientData struct {
	ConnectedNode          string `json:"node"`
	ClientID               string `json:"clientid"`
	Username               string `json:"username"`
	ProtocolName           string `json:"proto_name"`
	ProtocolVersion        int    `json:"proto_ver"`
	IPAddress              string `json:"ip_address"`
	Port                   int    `json:"port"`
	IsBridge               bool   `json:"is_bridge"`
	ConnectedAt            string `json:"connected_at"`
	DisconnectedAt         string `json:"disconnected_at"`
	IsConnected            bool   `json:"connected"`
	Zone                   string `json:"zone"`
	KeepAlive              int    `json:"keepalive"`
	CleanStart             bool   `json:"clean_start"`
	ExpiryInterval         int    `json:"expiry_interval"`
	CreatedAt              string `json:"created_at"`
	SubscriptionCount      int    `json:"subscriptions_cnt"`
	MaxSubscriptions       int    `json:"max_subscriptions"`
	InFlight               int    `json:"inflight"`
	MaxInFlight            int    `json:"max_inflight"`
	MessageQueueLength     int    `json:"mqueue_len"`
	MaxMessageQueueLength  int    `json:"max_mqueue"`
	MessageQueueDropped    int    `json:"mqueue_dropped"`
	AwaitingRel            int    `json:"awaiting_rel"`
	MaxAwaitingRel         int    `json:"max_awaiting_rel"`
	BytesReceived          int    `json:"recv_oct"`
	MqttPacketsReceived    int    `json:"recv_pkt"`
	PublishPacketsRecieved int    `json:"recv_msg"`
	BytesSent              int    `json:"send_oct"`
	TCPPacketsSent         int    `json:"send_cnt"`
	MqttPacketsSent        int    `json:"send_pkt"`
	PublishPacketsSent     int    `json:"send_msg"`
	MailboxLength          int    `json:"mailbox_len"`
	HeapSize               int    `json:"heap_size"`
	Reductions             int    `json:"reductions"`
}

// MqttMeta received with API requested
type MqttMeta struct {
	Page  int `json:"page"`
	Limit int `json:"limit"`
	Count int `json:"count"`
}

// GetClientResponse response for retreiving client
type GetClientResponse struct {
	Code int              `json:"code"`
	Data []MqttClientData `json:"data"`
	Meta MqttMeta         `json:"meta"`
}
