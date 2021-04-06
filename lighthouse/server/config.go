package server

import "crypto/ed25519"

// Config ...
type Config struct {
	// Hostname of where broker should listen
	//
	// Default: "0.0.0.0"
	Hostname string

	// Port of where broker should listen
	//
	// Default: "997"
	Port uint32

	// Ed25519 Private key
	//
	// Required
	PrivateKey ed25519.PrivateKey

	// Ed25519 Public key
	//
	// Required
	PublicKey ed25519.PublicKey
}

// Parse parses options and set defaults
func (cfg Config) Parse() Config {
	if cfg.Hostname == "" {
		cfg.Hostname = "0.0.0.0"
	}
	if cfg.Port == 0 {
		cfg.Port = 997
	}

	return cfg
}
