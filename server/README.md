# Houseflow API server

API Server for Houseflow. Built using [axum](https://github.com/tokio-rs/axum), a web framework.

## Clients

The server currently supports two type of clients:

- Internal, e.g CLI app
- Google Home

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
