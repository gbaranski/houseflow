package main

import (
	"crypto/ed25519"
	"crypto/rand"

	"github.com/gbaranski/houseflow/lighthouse/http_server"
	"github.com/gbaranski/houseflow/lighthouse/tcp_server"
)

func main() {
	pkey, skey, err := ed25519.GenerateKey(rand.Reader)
	if err != nil {
		panic(err)
	}
	tcpServer := tcp_server.New(tcp_server.Config{
		Hostname:   "0.0.0.0",
		Port:       3030,
		PrivateKey: skey,
		PublicKey:  pkey,
	})

	go func() {
		err = tcpServer.Run()
		if err != nil {
			panic(err)
		}
	}()

	httpServer := http_server.New(http_server.Config{
		Hostname: "0.0.0.0",
		Port:     80,
	}, &tcpServer.SessionStore)
	go func() {
		println("Starting HTP")
		err = httpServer.Run()
		if err != nil {
			panic(err)
		}
	}()

	select {}

}
