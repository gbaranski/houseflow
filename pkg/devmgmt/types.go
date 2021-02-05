package devmgmt

import "errors"

// ResponseTopic is type of topic which contains Request and Response
type ResponseTopic struct {
	Request  string
	Response string
}

// ErrDeviceTimeout indicates that device had timeout
var ErrDeviceTimeout = errors.New("device timeout")

// ErrInvalidSignature indicates that device sent back invalid signature of payload
var ErrInvalidSignature = errors.New("invalid signature")
