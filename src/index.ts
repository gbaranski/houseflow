/* eslint-disable no-console */
import express from 'express';
import cors from 'cors';
import './firebase';
import Alarmclock from './alarmclock';
import { AlarmRequestType, WaterRequestType } from './types';
import { isAuthenticated } from './auth';
import { waterMixerHandleRequest, waterMixerFetchEspDataInterval } from './watermixer';

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

app.post('/alarmclock/getData', (req, res) => {
  alarmClock.handleRequest(req, res, AlarmRequestType.GET_DATA);
});

app.post('/alarmclock/getTempArray', (req, res) => {
  alarmClock.handleRequest(req, res, AlarmRequestType.GET_TEMP_ARRAY);
});

app.post('/alarmclock/testSiren', (req, res) => {
  alarmClock.handleRequest(req, res, AlarmRequestType.TEST_ALARM);
});

app.post('/alarmclock/setTime', (req, res) => {
  alarmClock.handleRequest(req, res, AlarmRequestType.SET_TIME);
});

app.post('/alarmclock/switchState', (req, res) => {
  alarmClock.handleRequest(req, res, AlarmRequestType.SWITCH_STATE);
});

app.post('/watermixer/start', (req, res) => {
  waterMixerHandleRequest(req, res, WaterRequestType.START_MIXING);
});

app.post('/watermixer/getData', (req, res) => {
  waterMixerHandleRequest(req, res, WaterRequestType.GET_DATA);
});

app.listen(httpPort, () => console.log(`Example app listening at http://localhost:${httpPort}`));
