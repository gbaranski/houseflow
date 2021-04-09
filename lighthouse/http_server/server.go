package http_server

import (
  "fmt"
	"github.com/gbaranski/houseflow/lighthouse/tcp_server"
	"github.com/google/uuid"
	"github.com/sirupsen/logrus"
	"net/http"
	"strings"
)

type HttpServer struct {
	sessionStore *tcp_server.SessionStore
	cfg          Config
}

func New(cfg Config, sessionStore *tcp_server.SessionStore) HttpServer {
	server := HttpServer{
		sessionStore: sessionStore,
		cfg:          cfg,
	}
	http.HandleFunc("/execute/", server.onExecute)

	return server
}

func (s *HttpServer) Run() error {
	logrus.WithFields(logrus.Fields{
		"hostname": s.cfg.Hostname,
		"port":     s.cfg.Port,
	}).Info("Listening for incoming TCP connections")

	err := http.ListenAndServe("0.0.0.0:8080", nil)
	if err != nil {
		return err
	}

	return nil
}

func (s *HttpServer) onExecute(w http.ResponseWriter, r *http.Request) {
	logrus.Infoln("Received execute request")
	pathParts := strings.Split(r.URL.Path, "/")
	if len(pathParts) < 3 {
		w.WriteHeader(400)
		return
	}

	clientIDString := pathParts[2]
  clientID, err := uuid.Parse(clientIDString)
	if err != nil {
		w.WriteHeader(400)
		return
	}
	logrus.Infof("ClientID: %s", clientID)

}
