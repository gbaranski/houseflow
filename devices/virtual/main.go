package main

import (
	"github.com/gbaranski/houseflow/devices/virtual/config"
	"github.com/gbaranski/houseflow/devices/virtual/device"
)

func main() {
	config, err := config.Load()
	if err != nil {
		panic(err)
	}
	device := device.CreateDevice(config)
	err = device.StartTCP()
	if err != nil {
		panic(err)
	}

	select {}
}
