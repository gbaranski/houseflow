package server

import (
	"crypto/ed25519"
	"crypto/rand"
	"net"
	"testing"
)

func TestSend(t *testing.T) {
	pkey, skey, err := ed25519.GenerateKey(rand.Reader)
	if err != nil {
		t.Fatalf(err.Error())
	}
	b, err := New(Config{
		Hostname:   "localhost",
		Port:       1950,
		PrivateKey: skey,
		PublicKey:  pkey,
	})
	if err != nil {
		t.Fatalf(err.Error())
	}
	_, ipv4Net, err := net.ParseCIDR("192.0.2.1/24")

	client := Client{
		ID:        "SomeClientID",
		IPAddress: ipv4Net,
	}

	b.onSend(packet{
		Client: client,
		Writer: nil,
	})

}
