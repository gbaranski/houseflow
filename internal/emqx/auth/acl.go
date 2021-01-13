package auth

import "net/http"

// ACLRequest is sub/pub req
type ACLRequest struct {
	Access     string `json:"access"`
	Username   string `json:"username"`
	ClientID   string `json:"clientID"`
	IP         string `json:"ip"`
	Topic      string `json:"topic"`
	MountPoint string `json:"mountpoint"`
}

func (a *Auth) onACL(w http.ResponseWriter, req *http.Request) {
	// c.Status(200)
}
