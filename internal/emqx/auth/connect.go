package auth

import (
	"crypto/ed25519"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
)

const (
	// 4 * ceil(ED25519_PKEY_LENGTH / 3) = 44
	ed25519PublicKeyBase64Size = 44
)

// ConnectRequest is request about user connect
type ConnectRequest struct {
	ClientID string `json:"clientID"`
	IP       string `json:"ip"`
	Username string `json:"username"`
	Password string `json:"password"`
}

func (a *Auth) onServiceConnect(w http.ResponseWriter, req *http.Request, r ConnectRequest, pkey ed25519.PublicKey) {
	decodedSignature, err := base64.StdEncoding.DecodeString(r.Password)
	if err != nil {
		w.WriteHeader(http.StatusUnauthorized)
		w.Write([]byte(err.Error()))
	}

	valid := ed25519.Verify(ed25519.PublicKey(a.opts.ServerPublicKey), pkey, decodedSignature)
	if valid {
		w.Write([]byte("Authorized"))
		log.Printf("Connection: Service '%s' successfully authenticated\n", r.ClientID)
	} else {
		w.WriteHeader(http.StatusUnauthorized)
		w.Write([]byte("Unauthorized"))
		log.Printf("Connection: Service '%s' attempted to connect with invalid signature\n", r.ClientID)
	}
}

func (a *Auth) onDeviceConnect(w http.ResponseWriter, req *http.Request, r ConnectRequest, pkey ed25519.PublicKey) {
	device, err := a.db.GetDeviceByID(req.Context(), r.ClientID)
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		w.Write([]byte(err.Error()))
		return
	}
	if device.PublicKey != r.Username {
		w.WriteHeader(http.StatusUnauthorized)
		w.Write([]byte("Username invalid"))
		return
	}
	decodedSignature, err := base64.StdEncoding.DecodeString(r.Password)
	if err != nil {
		w.WriteHeader(http.StatusUnprocessableEntity)
		w.Write([]byte(err.Error()))
		return
	}

	valid := ed25519.Verify(pkey, pkey, decodedSignature)
	if valid {
		w.Write([]byte("Authorized"))
		log.Printf("Connection: Device %s authenticated\n", r.ClientID)
	} else {
		w.WriteHeader(http.StatusUnauthorized)
		w.Write([]byte("Unauthorized"))
		log.Printf("Connection: Device %s attempted to connect with invalid signature\n", r.ClientID)
	}
}

func (a *Auth) onConnect(w http.ResponseWriter, req *http.Request) {
	var r ConnectRequest
	err := json.NewDecoder(req.Body).Decode(&r)
	if err != nil {
		w.WriteHeader(http.StatusUnprocessableEntity)
		w.Write([]byte(err.Error()))
		return
	}
	if len(r.Username) != ed25519PublicKeyBase64Size {
		w.WriteHeader(http.StatusBadRequest)
		w.Write([]byte("invalid username"))
		return
	}

	pkey, err := base64.StdEncoding.DecodeString(r.Username)
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		w.Write([]byte(fmt.Sprintf("fail decode base64, %s", err.Error())))
		return
	}
	if a.opts.ServerPublicKey.Equal(ed25519.PublicKey(pkey)) {
		a.onServiceConnect(w, req, r, pkey)
	} else {
		a.onDeviceConnect(w, req, r, pkey)
	}

}
