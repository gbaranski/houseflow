# Houseflow 
[![commit-weekly](https://img.shields.io/github/commit-activity/w/gbaranski/houseflow)](https://github.com/gbaranski/houseflow)
[![lines-of-code](https://img.shields.io/tokei/lines/github/gbaranski/houseflow)](https://github.com/gbaranski/houseflow)
[![stars](https://img.shields.io/github/stars/gbaranski/houseflow?style=social)](https://github.com/gbaranski/houseflow)


Houseflow is open source home automation system, it lets you configure various devices like lights, switches, gates, sensors, and much more.

## Features

- [x] Fast, easy to scale due to [the architecture](#architecture)
- [x] CLI Client
- [x] Integration with Google Home
- [x] Linux support on x86-64 architecture
- [ ] Windows support(issue [#160](https://github.com/gbaranski/houseflow/issues/160))
- [x] ESP8266 Support 
- [ ] ESP32 Support(issue [#161](https://github.com/gbaranski/houseflow/issues/161))

## Architecture

<img src="/docs/architecture.svg">

## Server

Houseflow server is splitted into few parts.

- Auth, responsible for handling user logging in, signing up, refreshing access tokens, handling OAuth2 from other applications.
- Fulfillment, responsible for handling requests device requests from WAN, e.g execute some command on a device, query state of the device and etc. It also handles requests from third-party services like Google Actions.
- Lighthouse, responsible for allowing devices to connect from outside network and provide HTTP JSON RPC for the [fulfillment service](#fulfillment). Uses websockets for communication with devices.

## Client

Houseflow is designed to have many clients, and if need easily add new third-party services like Google Home, currently there are few clients supported:

- Internal CLI app, located at [`core/`](./core)
- [Google Home](https://developers.google.com/assistant/smarthome/overview)

## Device

## ESP8266/ESP32

Written using Arduino framework and PlatformIO.

## Raspberry Pi

Supported via the [`devices/virtual/`](devices/virtual) crate.

## Contributing
Pull requests are welcome
