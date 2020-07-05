import express from 'express';

const router = express.Router();

app.get('/', (req, res) => {
  res.send('Hello from API server');
});

app.post('/login', (req, res) => {
  const username = req.header('username');
  const password = req.header('password');
  if (username && password) {
    if (isAuthenticated(username, password)) {
      res.send(200).end();
    } else {
      res.send(401).end();
    }
  } else {
    res.send(401).end();
  }
});
app.get('/getHistory', (req, res) => {
  if (!isAuthenticated(req.header('username') || '', req.header('password') || '')) {
    res.send(401).end();
  }
  res.json(getHistory());
});
app.get('/getDeviceStatus', (req, res) => {
  if (!isAuthenticated(req.header('username') || '', req.header('password') || '')) {
    res.send(401).end();
  }
  res.json(JSON.stringify(deviceStatus));
});

app.post('/alarmclock/getData', (req, res) => {
  AlarmclockHandleRequest(req, res, AlarmRequestType.GET_DATA);
});

app.post('/alarmclock/getTempArray', (req, res) => {
  AlarmclockHandleRequest(req, res, AlarmRequestType.GET_TEMP_ARRAY);
});

app.post('/alarmclock/testSiren', (req, res) => {
  AlarmclockHandleRequest(req, res, AlarmRequestType.TEST_ALARM);
  const reqHistory: RequestHistory = {
    user: req.header('username') || '',
    requestType: AlarmRequestType.TEST_ALARM,
    date: new Date(),
    ip: getIpStr(req),
  };
  createHistory(reqHistory);
});

app.post('/alarmclock/setTime', (req, res) => {
  AlarmclockHandleRequest(req, res, AlarmRequestType.SET_TIME);
  const reqHistory: RequestHistory = {
    user: req.header('username') || '',
    requestType: AlarmRequestType.SET_TIME,
    date: new Date(),
    ip: getIpStr(req),
  };
  createHistory(reqHistory);
});

app.post('/alarmclock/switchState', (req, res) => {
  AlarmclockHandleRequest(req, res, AlarmRequestType.SWITCH_STATE);
  const reqHistory: RequestHistory = {
    user: req.header('username') || '',
    requestType: AlarmRequestType.SWITCH_STATE,
    date: new Date(),
    ip: getIpStr(req),
  };
  createHistory(reqHistory);
});

app.post('/watermixer/start', (req, res) => {
  waterMixerHandleRequest(req, res, WaterRequestType.START_MIXING);
  const reqHistory: RequestHistory = {
    user: req.header('username') || '',
    requestType: WaterRequestType.START_MIXING,
    date: new Date(),
    ip: getIpStr(req),
  };
  createHistory(reqHistory);
});

app.post('/watermixer/getData', (req, res) => {
  waterMixerHandleRequest(req, res, WaterRequestType.GET_DATA);
});
