package http_server

// Config ...
type Config struct {
	// Hostname of where broker should listen
	//
	// Default: "0.0.0.0"
	Hostname string

	// Port of where broker should listen
	//
	// Default: "80"
	Port uint32
}

// Parse parses options and set defaults
func (cfg Config) Parse() Config {
	if cfg.Hostname == "" {
		cfg.Hostname = "0.0.0.0"
	}
	if cfg.Port == 0 {
		cfg.Port = 80
	}

	return cfg
}
