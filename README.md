# Houseflow 
[![commit-weekly](https://img.shields.io/github/commit-activity/w/gbaranski/houseflow)](https://github.com/gbaranski/houseflow)
[![lines-of-code](https://img.shields.io/tokei/lines/github/gbaranski/houseflow)](https://github.com/gbaranski/houseflow)
[![stars](https://img.shields.io/github/stars/gbaranski/houseflow?style=social)](https://github.com/gbaranski/houseflow)


Houseflow is open source home automation system, it lets you configure various devices like lights, switches, gates, sensors, and much more.

## Features

- [x] Fast, written with high-performance languague, Rust.
- [ ] Easy to use([issues with #ux label](https://github.com/gbaranski/houseflow/issues?q=is%3Aissue+label%3Aux+))
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

## Getting started

### Installation

Installing using Cargo is recommended since it can speed up compilation by disabling default features.

#### Cargo

```bash
$ cargo install houseflow
```

Cargo can also build only specific features, while every other installation source, like from AUR builds all features. There are three features available during installation:
- `client` for commands like `houseflow fulfillment`, or `houseflow auth`.
- `server` for `houseflow server` command.
- `device` for `houseflow device` command.

##### Server only

```bash
$ cargo install --no-default-features --feature "server" houseflow
```

##### Client and Device only

```bash
$ cargo install --no-default-features --feature "client,device" houseflow
```

#### Arch Linux

There is [AUR](https://aur.archlinux.org/packages/houseflow-git/) package available.

### Generating configuration

Generate new configuration with
```bash
$ houseflow config generate
```


### Server setup
**Only for installation using Cargo**: make sure that houseflow is built with `server` feature.

##### PostgreSQL setup

1. Install PostgreSQL
2. Run initial configuration, more info on [Arch Wiki](https://wiki.archlinux.org/title/PostgreSQL#Initial_configuration)
3. Start `postgres` service, if using systemd
```bash
$ sudo systemctl start postgres
```
4. Create new database named `houseflow`.
```bash
$ createdb houseflow
```

##### Redis setup

1. Install Redis
2. Start `postgres` service, if using systemd
```bash
$ sudo systemctl start postgres
```

##### Starting server
Start server using

```bash
$ houseflow server run
```
this should start server at port `6001`.

##### Reachability from outside network

Houseflow server won't be available outside localhost by default due to security concerns, to make it available, open `~/.config/houseflow/server.toml` and change `address` to `0.0.0.0:6001`.

### Device setup

##### Registering device

TODO: https://github.com/gbaranski/houseflow/issues/162

#### Starting device

If server is running at `127.0.0.1:6001`

```bash
$ houseflow device run
```
otherwise open `~/.config/houseflow/device.toml`, and change `address` to address of the server(Hint: server port also must be present, default port is `6001`).

### Client setup

#### Creating user

Start by registering user, you'll be prompted for credentials
```bash
$ houseflow auth register
```

Then log in into the account
```bash
$ houseflow auth login
```

Retrieve ID of currently logged user
```bash
$ houseflow auth status
```

Give user access to specific device with read, write, execute
```bash
$ houseflow admin permit <user-id> <device-id> --read --write --execute
```

#### Manage user devices

Start by syncing allowed devices with fulfillment
```bash
$ houseflow fulfillment sync
```

Query state of the device
```bash
$ houseflow fulfillment query <device-id>
```

## Contributing
Contributors are very welcome! **No contribution is too small and all contributions are valued.**

## Getting help

Get in touch with me on Matrix @gbaranski:matrix.org, or via email root@gbaranski.com.
