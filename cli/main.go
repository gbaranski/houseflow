package main

import (
	"os"

	"fmt"
	"time"

	"github.com/gbaranski/houseflow/cli/genkey"
)

func showUsageAndExit() {
	fmt.Println(`Usage: houseflow <command>

Available commands: 
	genkey 
	   Generates Ed25519 public/private key
	   Example: houseflow genkey
		`)
	os.Exit(0)
}

func main() {
	if len(os.Args) < 2 {
		showUsageAndExit()
	}
	action := os.Args[1]

	start := time.Now()
	if action == "genkey" {
		pkey, skey := genkey.GenerateKeypair()
		pkeyEncoded, skeyEncoded := genkey.EncodeKeypair(pkey, skey)
		pkeyDecoded, skeyDecoded := genkey.DecodeKeypair(pkeyEncoded, skeyEncoded)
		genkey.ValidateKeypairsEquality(genkey.Keypair{
			PublicKey:  pkey,
			PrivateKey: skey,
		}, genkey.Keypair{
			PublicKey:  pkeyDecoded,
			PrivateKey: skeyDecoded,
		})

		genkey.ValidateSigningPublicKey(pkey, skey)
		genkey.ValidateSigningPublicKey(pkeyDecoded, skeyDecoded)

		genkey.ValidateSigningRandomMessage(pkey, skey)
		genkey.ValidateSigningRandomMessage(pkeyDecoded, skeyDecoded)
		fmt.Printf("Public key: %s\n", pkeyEncoded)
		fmt.Printf("Private key: %s\n", skeyEncoded)

	} else {
		showUsageAndExit()
	}

	fmt.Printf("\nFinished after %d µs\n", time.Since(start).Microseconds())
}
