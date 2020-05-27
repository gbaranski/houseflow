/* eslint-disable no-console */
import express from 'express';
import Alarmclock from './alarmclock';
import Watermixer from './watermixer';
import { AlarmRequestType, WaterRequestType } from './types';

const httpPort = 8080;

const app = express();
const alarmClock = new Alarmclock();
const waterMixer = new Watermixer();
app.use(express.json()); // for parsing application/json

// app.post('/getAlarmClock', (req, res) => {
//   console.log(req.body);
//   res.json(req.body);
// });

setInterval(async () => {
  alarmClock.fetchEspDataInterval();
}, 1000);

setInterval(async () => {
  waterMixer.fetchEspDataInterval();
}, 1000);

app.post('/getAlarmclockData', (req, res) => {
  alarmClock.handleRequest(req, res, AlarmRequestType.GET_DATA);
});

app.post('/testAlarmclock', (req, res) => {
  alarmClock.handleRequest(req, res, AlarmRequestType.TEST_ALARM);
});

app.post('/setAlarmTime', (req, res) => {
  alarmClock.handleRequest(req, res, AlarmRequestType.SET_TIME);
});

app.post('/switchAlarmState', (req, res) => {
  alarmClock.handleRequest(req, res, AlarmRequestType.SWITCH_STATE);
});

app.post('/startMixing', (req, res) => {
  waterMixer.handleRequest(req, res, WaterRequestType.START_MIXING);
});

app.listen(httpPort, () => console.log(`Example app listening at http://localhost:${httpPort}`));
