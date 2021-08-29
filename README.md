# Houseflow

[![build](https://img.shields.io/github/workflow/status/gbaranski/houseflow/CI)](https://github.com/gbaranski/houseflow/actions?query=workflow%3ACI)
[![lines-of-code](https://img.shields.io/tokei/lines/github/gbaranski/houseflow)](https://github.com/gbaranski/houseflow)

Houseflow is open source home automation system, it lets you configure various devices like lights, switches, gates, sensors, and much more.

# Features

- [x] Fast, written with high-performance languague, Rust.
- [x] Integration with Google Home
- [x] Configurable from config files.
- [x] CLI Client
- [ ] Easy to use([issues with #ux label](https://github.com/gbaranski/houseflow/issues?q=is%3Aissue+label%3Aux+))
- [x] ESP8266 Support 
- [ ] ESP32 Support(issue [#161](https://github.com/gbaranski/houseflow/issues/161))

# Architecture

<img src="./docs/architecture.svg">

## Server

Houseflow server is splitted into few parts.

- Auth, responsible for handling user logging in, signing up, refreshing access tokens.
- OAuth2, handles OAuth2 requests from Google, used for integration with Google Home.
- Fulfillment. Handles all device related requests from users, such as EXECUTE, QUERY, or SYNC. Supports multiple clients, see [Client](#Client). Uses Lighthouse to send the requests.
- Lighthouse. A websocket server which connects with embedded devices.

## Client

Houseflow is designed to have many clients. At the moment only 2 clients are supported.

- CLI app, located at [`cli/`](./cli)
- [Google Home](https://developers.google.com/assistant/smarthome/overview)

## Device

### ESP8266/ESP32

Written using Arduino framework and PlatformIO.

### Raspberry Pi

Supported via the [`devices/virtual/`](devices/virtual) crate.

# Crates

Houseflow exposes many crates. Here's a brief overview of what they're for.
- `houseflow-api`. API for communicating with `houseflow-server`.
- `houseflow-cli`. CLI Interface for communicating with `houseflow-server`. Uses `houseflow-api` under the hood.
- `houseflow-config`. Type definitions of Houseflow configuration files, and utilities for reading them.
- `houseflow-db`. Database interface & implementations, currently supported is SQLite. It's used only for storing users and admins.
- `houseflow-device`. Blocks for building embedded devices.
- `google-smart-home`. Google Smart Home type definitions, they match with [the official source](https://developers.google.com/assistant/smarthome/overview).
- `houseflow-server`. Houseflow server, can be used as a library or as a binary.
- `houseflow-types`. Type definitions for Houseflow, such as Devices, Users, Tokens and etc.

# Contributing
Contributors are very welcome! **No contribution is too small and all contributions are valued.**

# Getting help

Get in touch with me on Matrix @gbaranski:matrix.org, or via email root@gbaranski.com.
