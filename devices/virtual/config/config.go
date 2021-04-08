package config

import (
	"crypto/ed25519"
	"encoding/base64"
	"fmt"
	"os"
	"strconv"

	"github.com/google/uuid"
	"github.com/joho/godotenv"
)

type topic struct {
	Request  string
	Response string
}

// Config is runtime configuration, PublicKey and PrivateKey are loeaded from .env or env variables
type Config struct {
	PublicKey       ed25519.PublicKey
	PrivateKey      ed25519.PrivateKey
	ServerPublicKey ed25519.PublicKey
	DeviceID        uuid.UUID
	ServerHost      string
	ServerPort      uint16
}

// Load loads config and returns it
func Load() (*Config, error) {
	err := godotenv.Load()
	if err != nil {
		return nil, err
	}
	pkeystr, exists := os.LookupEnv("PUBLIC_KEY")
	if !exists {
		return nil, fmt.Errorf("PUBLIC_KEY does not exist in .env")
	}
	pkeyDecoded, err := base64.StdEncoding.DecodeString(pkeystr)
	if err != nil {
		return nil, fmt.Errorf("PUBLIC_KEY is invalid %s", err.Error())
	}
	pkey := ed25519.PublicKey(pkeyDecoded)

	skeystr, exists := os.LookupEnv("PRIVATE_KEY")
	if !exists {
		return nil, fmt.Errorf("PRIVATE_KEY does not exist in .env")
	}
	skeyDecoded, err := base64.StdEncoding.DecodeString(skeystr)
	if err != nil {
		return nil, fmt.Errorf("PRIVATE_KEY is invalid %s", err.Error())
	}
	skey := ed25519.PrivateKey(skeyDecoded)

	serverPkeyStr, exists := os.LookupEnv("SERVER_PUBLIC_KEY")
	if !exists {
		return nil, fmt.Errorf("SERVER_PUBLIC_KEY does not exist in .env")
	}
	serverPkeyDecoded, err := base64.StdEncoding.DecodeString(serverPkeyStr)
	if err != nil {
		return nil, fmt.Errorf("SERVER_PUBLIC_KEY is invalid: %s", err.Error())
	}
	serverPkey := ed25519.PublicKey(serverPkeyDecoded)

	deviceID, exists := os.LookupEnv("DEVICE_ID")
	if !exists {
		return nil, fmt.Errorf("DEVICE_ID does not exist in .env")
	}

	serverHost, exists := os.LookupEnv("SERVER_HOST")
	if !exists {
		return nil, fmt.Errorf("SERVER_HOST does not exist in .env")
	}

	serverPortString, exists := os.LookupEnv("SERVER_PORT")
	if !exists {
		return nil, fmt.Errorf("SERVER_PORT does not exist in .env")
	}
	serverPort, err := strconv.ParseInt(serverPortString, 10, 32)

	return &Config{
		PublicKey:       pkey,
		PrivateKey:      skey,
		ServerPublicKey: serverPkey,
		DeviceID:        uuid.MustParse(deviceID),
		ServerPort:      uint16(serverPort),
		ServerHost:      serverHost,
	}, nil
}
