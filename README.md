# Control-home API server
Start server
```
sudo chmod +x start.sh
./start.sh
```

Manually
```
docker run -p 8000:8000 -d gbaranski/control-api
```

All requests must include username and password headers
Verify username and password
```
POST /api/login
```
# Alarmclock
Get data // To change to GET request
```
POST /api/alarmclock/getData

```
Get temperatures array
```
POST /api/alarmclock/getTempArray
```
Test alarmclock siren
```
POST /api/alarmclock/testSiren
```
Set time
```
POST /api/alarmclock/setTime
headers:
- time
```
Switch state
```
POST /api/alarmclock/switchState
headers:
- state
```
# Watermixer
Start mixing
```
POST /api/watermixer/start
```
Get data
```
POST /api/watermixer/getData
```
