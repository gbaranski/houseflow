package devmgmt

import "crypto/ed25519"

// Options of the Devmgmt
type Options struct {
	// ClientID, required
	ClientID string

	// Default: "tcp://broker:1883"
	BrokerURL string

	// ServerPublicKey is servers public key
	//
	// *Required*
	ServerPublicKey ed25519.PublicKey

	// ServerPrivateKey is servers private key
	//
	// *Required*
	ServerPrivateKey ed25519.PrivateKey
}

// Parse parses options to the defaults
func (opts *Options) Parse() {
	if opts.BrokerURL == "" {
		opts.BrokerURL = "tcp://broker:1883"
	}
	if opts.ServerPublicKey == nil {
		panic("ServerPublicKey option is required")
	}
	if opts.ServerPrivateKey == nil {
		panic("ServerPrivateKey option is required")
	}
}
