# Houseflow API server

API Server for Houseflow. Built using [axum](https://github.com/tokio-rs/axum), a web framework.

## Clients

The server currently supports two type of clients:

- Internal, e.g CLI app
- Google Home

## Backends

Devices may come from one of two sources, configured per user:

- Lighthouse, a custom protocol defined by Houseflow.
- [Homie](docs/homie.md), a [convention for IoT devices](https://homieiot.github.io/) defined on top
  of MQTT.

## Code organisation

The main modules and directories are:

### auth/

Handles internal clients authorization.

### fulfillment/

Fulfillment service supports following intents

- Sync, get all available devices for a user.
- Query, used to check device state.
- Execute, used to execute some command on device, e.g turn on lights.
- Disconnect (Google Home only), disconnects user from a service.

### lighthouse/

Handles websocket connections with embedded devices.

### oauth/

Handles OAuth2 requests from G-Home.

### token_blacklist/

Store for refresh tokens, used to invalidate them in case something happens.
[Sled](https://github.com/spacejam/sled) is used by default, but any database is supported because
of available `TokenBlacklist` interface.

### extractors.rs

Axum extractors for things such as RefreshToken or AccessToken.
