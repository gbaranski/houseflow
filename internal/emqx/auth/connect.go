package auth

import (
	"crypto/ed25519"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"log"
	"net/http"

	"go.mongodb.org/mongo-driver/bson/primitive"
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

func (a *Auth) onServiceConnect(w http.ResponseWriter, req *http.Request, r ConnectRequest, pkey []byte) {
	encodedSignature, err := base64.StdEncoding.DecodeString(r.Password)
	if err != nil {
		w.WriteHeader(http.StatusUnauthorized)
		w.Write([]byte(err.Error()))
	}

	valid := ed25519.Verify(ed25519.PublicKey(a.opts.ServerPublicKey), pkey, encodedSignature)
	if valid {
		w.Write([]byte("Authorized"))
		log.Printf("Service '%s' successfully authenticated\n", r.ClientID)
	} else {
		w.WriteHeader(http.StatusUnauthorized)
		w.Write([]byte("Unauthorized"))
		log.Printf("Service '%s' attempted to connect with invalid signature\n", r.ClientID)
	}
}

func (a *Auth) onDeviceConnect(w http.ResponseWriter, req *http.Request, r ConnectRequest) {
	deviceID, err := primitive.ObjectIDFromHex(r.Username)
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		w.Write([]byte(err.Error()))
		return
	}
	device, err := a.db.GetDeviceByID(req.Context(), deviceID)
	if err != nil {
		w.WriteHeader(http.StatusNotFound)
		w.Write([]byte(err.Error()))
		return
	}
	valid := ed25519.Verify(ed25519.PublicKey(device.PublicKey), []byte(device.PublicKey), []byte(r.Password))
	if valid {
		w.Write([]byte("Authorized"))
	} else {
		w.WriteHeader(http.StatusUnauthorized)
		w.Write([]byte("Unauthorized"))
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

	fmt.Printf("User request: %+v\n", r)
	pkey, err := base64.StdEncoding.DecodeString(r.Username)
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		w.Write([]byte(fmt.Sprintf("fail decode base64, %s", err.Error())))
		return
	}
	if a.opts.ServerPublicKey.Equal(ed25519.PublicKey(pkey)) {
		fmt.Println("CHecking as for service")
		a.onServiceConnect(w, req, r, pkey)
	} else {
		fmt.Println("CHecking as for device")
		a.onDeviceConnect(w, req, r)
	}

}
