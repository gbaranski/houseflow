# Control-home API server
Start server
```
sudo chmod +x start.sh
./start.sh
```

Manually
```
docker run -p 8000:8000 -d .
```

All requests must include username and password headers
Verify username and password
```
POST /api/login
```

Get device status
```
GET /getDeviceStatus
```
Get requests history
```
GET /getHistory
```

# Alarmclock
Get data // To change to GET request
```
POST /alarmclock/getData

```
Get temperatures array
```
POST /alarmclock/getTempArray
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
