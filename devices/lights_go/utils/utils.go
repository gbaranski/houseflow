package utils

import (
	"encoding/base64"
	"fmt"
	"strings"
)

// ParsePayload parses payload, returns respectively payload and signature
func ParsePayload(p []byte) (string, []byte, error) {
	splitted := strings.SplitN(string(p), ".", 2)
	if len(splitted) < 1 {
		return "", nil, fmt.Errorf("payload is invalid, it should contain payload and signature")
	}
	signature := splitted[0]
	decoded, err := base64.StdEncoding.DecodeString(signature)
	if err != nil {
		return "", nil, fmt.Errorf("failed parsing signature %s", err.Error())
	}
	payload := splitted[1]

	return payload, decoded, nil

}
