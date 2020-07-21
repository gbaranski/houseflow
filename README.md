# Control-home API server
`docker pull gbaranski19/control-home-api:latest`
`docker-compose up`

***All requests must include username and password headers***

Verify username and password
```
POST /api/login
```

Get token for websocket connections
```
GET /getToken
- device: string
- token: uniqueDeviceToken
```


Get device status
```
GET /api/getDeviceStatus
```

Get websocket clients
```
GET /getClients
```

# Alarmclock
Get data // To change to GET request
```
POST /alarmclock/getData

```
Test alarmclock siren
```
POST /alarmclock/testSiren
```
Set time
```
POST /alarmclock/setTime
headers:
- time
```
Switch state
```
POST /alarmclock/switchState
headers:
- state
```
# Watermixer
Start mixing
```
POST /watermixer/start
```
Get data
```
POST /watermixer/getData
```
