package auth

import (
	"crypto/ed25519"
	"encoding/base64"
	"encoding/json"
	"net/http"
)

// ACLRequest is sub/pub req
type ACLRequest struct {
	// 1 | 2
	// 1 = sub
	// 2 = pub
	Access     byte   `json:"access"`
	Username   string `json:"username"`
	ClientID   string `json:"clientID"`
	IP         string `json:"ip"`
	Topic      string `json:"topic"`
	MountPoint string `json:"mountpoint"`
}

func (a *Auth) onACL(w http.ResponseWriter, req *http.Request) {
	var r ACLRequest
	err := json.NewDecoder(req.Body).Decode(&r)
	if err != nil {
		w.WriteHeader(http.StatusUnprocessableEntity)
		w.Write([]byte(err.Error()))
		return
	}
	pkey, err := base64.StdEncoding.DecodeString(r.Username)
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		w.Write([]byte(err.Error()))
		return
	}

	// Check if this client is some service
	if a.opts.ServerPublicKey.Equal(ed25519.PublicKey(pkey)) {
		w.Write([]byte("Authenticated"))
		return
	}
	topicClientID := r.Topic[0:24]
	if topicClientID != r.ClientID {
		w.WriteHeader(http.StatusUnauthorized)
		w.Write([]byte("Unauthorized for this topic"))
		return
	}

	w.Write([]byte("Authenticated"))
}
