package tcp_server

// Server ...
type Server struct {
	cfg         Config
	SessionStore SessionStore
}

// New creates new server and returns it
func New(cfg Config) Server {
	server := Server{
		cfg:         cfg.Parse(),
		SessionStore: NewSessionStore(),
	}
  return server
}
