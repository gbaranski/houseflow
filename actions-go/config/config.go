package config

import (
	"crypto/ed25519"
	"encoding/base64"
	"fmt"
	"os"
)

// Config is struct holding configuration
type Config struct {
	PublicKey  ed25519.PublicKey
	PrivateKey ed25519.PrivateKey
}

// Load loads configuration
func Load() (*Config, error) {
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
	fmt.Println(skeystr)
	skeySeedDecoded, err := base64.StdEncoding.DecodeString(skeystr)
	if err != nil {
		return nil, fmt.Errorf("PRIVATE_KEY is invalid %s", err.Error())
	}
	skey := ed25519.NewKeyFromSeed(skeySeedDecoded)

	return &Config{
		PublicKey:  pkey,
		PrivateKey: skey,
	}, nil

}
