/* eslint-disable no-console */
import express from 'express';
import cors from 'cors';
import './firebase';
import Alarmclock from './alarmclock';
import { AlarmRequestType, WaterRequestType, RequestHistory } from './types';
import { isAuthenticated } from './auth';
import { waterMixerHandleRequest, waterMixerFetchEspDataInterval } from './watermixer';
import { request } from 'http';
import { getHistory, createHistory } from './helpers';

if (!process.env.GBARANSKI) {
  throw new Error('missing env AUTH_KEY_GBARANSKI');
}
// export GOOGLE_APPLICATION_CREDENTIALS="/Users/gbaranski/code/firebase/firebase.json"
const httpPort = 8000;

const app = express();
const whitelist = ['https://control.gbaranski.com', 'http://localhost:3000', '*'];
app.use(cors({ origin: whitelist }));

const alarmClock = new Alarmclock();
app.use(express.json()); // for parsing application/json

// app.post('/getAlarmClock', (req, res) => {
//   console.log(req.body);
//   res.json(req.body);
// });

setInterval(async () => {
  // remove async
  alarmClock.fetchEspDataInterval();
}, 1000);

setInterval(async () => {
  // remove async
  waterMixerFetchEspDataInterval();
}, 1000);

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

app.post('/alarmclock/getData', (req, res) => {
  alarmClock.handleRequest(req, res, AlarmRequestType.GET_DATA);
});

app.post('/alarmclock/getTempArray', (req, res) => {
  alarmClock.handleRequest(req, res, AlarmRequestType.GET_TEMP_ARRAY);
});

app.post('/alarmclock/testSiren', (req, res) => {
  alarmClock.handleRequest(req, res, AlarmRequestType.TEST_ALARM);
  const reqHistory: RequestHistory = {
    user: req.header('username') || '',
    requestType: AlarmRequestType.TEST_ALARM,
    date: new Date(),
  };
  createHistory(reqHistory);
});

app.post('/alarmclock/setTime', (req, res) => {
  alarmClock.handleRequest(req, res, AlarmRequestType.SET_TIME);
  const reqHistory: RequestHistory = {
    user: req.header('username') || '',
    requestType: AlarmRequestType.SET_TIME,
    date: new Date(),
  };
  createHistory(reqHistory);
});

app.post('/alarmclock/switchState', (req, res) => {
  alarmClock.handleRequest(req, res, AlarmRequestType.SWITCH_STATE);
  const reqHistory: RequestHistory = {
    user: req.header('username') || '',
    requestType: AlarmRequestType.SWITCH_STATE,
    date: new Date(),
  };
  createHistory(reqHistory);
});

app.post('/watermixer/start', (req, res) => {
  waterMixerHandleRequest(req, res, WaterRequestType.START_MIXING);
  const reqHistory: RequestHistory = {
    user: req.header('username') || '',
    requestType: WaterRequestType.START_MIXING,
    date: new Date(),
  };
  createHistory(reqHistory);
});

app.post('/watermixer/getData', (req, res) => {
  waterMixerHandleRequest(req, res, WaterRequestType.GET_DATA);
});

app.listen(httpPort, () => console.log(`Example app listening at http://localhost:${httpPort}`));
