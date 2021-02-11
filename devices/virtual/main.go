package main

import (
	"github.com/gbaranski/houseflow/devices/lights_go/config"
	"github.com/gbaranski/houseflow/devices/lights_go/device"
)

func main() {
	config, err := config.Load()
	if err != nil {
		panic(err)
	}
	dev := device.CreateDevice(config)
	err = dev.StartMQTT()
	if err != nil {
		panic(err)
	}

	select {}
}
