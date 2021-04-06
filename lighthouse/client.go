package main

import (
	"bufio"
	"context"
	"crypto/ed25519"
	"crypto/rand"
	"os"
	"strings"
	"time"

	"github.com/gbaranski/houseflow/lighthouse/client"
	log "github.com/sirupsen/logrus"
)

func main() {
	pkey, skey, err := ed25519.GenerateKey(rand.Reader)
	if err != nil {
		panic(err)
	}
	c := client.New(client.Config{
		ClientID:   "someClientID",
		PublicKey:  pkey,
		PrivateKey: skey,
		Hostname:   "localhost",
		Port:       3030,
	})
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	err = c.Connect(ctx)
	if err != nil {
		panic(err)
	}
	cancel()

	log.Info("Successfully connected")

	r := bufio.NewReader(os.Stdin)
	for {
		text, _ := r.ReadString('\n')
		if text == "ping\n" {
			ctx, cancel = context.WithTimeout(context.Background(), time.Second)
			id, err := c.Ping(ctx)
			if err != nil {
				panic(err)
			}
			cancel()
			log.WithField("id", id).Info("Sent PING packet")
			continue
		}
		if strings.HasPrefix(text, "wres") {
			ctx, cancel = context.WithTimeout(context.Background(), time.Second)
			log.WithField("msg", text).Info("Sending message with expected response")
			_, err = c.SendWithResponse(ctx, []byte(text))
		} else {
			log.WithField("msg", text).Info("Sending message")
			err = c.Send([]byte(text))
		}
		if err != nil {
			log.WithError(err).Error("Fail send message")
		}
	}
}
