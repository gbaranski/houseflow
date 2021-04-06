package main

import (
	"crypto/ed25519"
	"crypto/rand"

	"github.com/gbaranski/houseflow/lighthouse/server"
)

func main() {
	pkey, skey, err := ed25519.GenerateKey(rand.Reader)
	if err != nil {
		panic(err)
	}
	b, err := server.New(server.Config{
		Hostname:   "0.0.0.0",
		Port:       3030,
		PrivateKey: skey,
		PublicKey:  pkey,
	})
	if err != nil {
		panic(err)
	}
	panic(b.ListenTCP())
}
