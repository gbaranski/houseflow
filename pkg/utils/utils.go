package utils

import (
	"encoding/base64"
	"fmt"
	"math/rand"
	"strings"
	"time"

	"golang.org/x/crypto/bcrypt"
)

// ParseSignedPayload parses payload, returns respectively message and signature
func ParseSignedPayload(p []byte) (string, []byte, error) {
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

// GenerateRandomString generates random string and returns it
func GenerateRandomString(length int) string {
	rand.Seed(time.Now().UnixNano())
	chars := []rune("ABCDEFGHIJKLMNOPQRSTUVWXYZ" +
		"abcdefghijklmnopqrstuvwxyz" +
		"0123456789")
	var b strings.Builder
	for i := 0; i < length; i++ {
		b.WriteRune(chars[rand.Intn(len(chars))])
	}
	return b.String()
}

// HashPassword hashes password with bcrypt
func HashPassword(pass string) ([]byte, error) {
	return bcrypt.GenerateFromPassword([]byte(pass), bcrypt.DefaultCost)
}
