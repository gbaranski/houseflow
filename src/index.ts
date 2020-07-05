/* eslint-disable no-console */
import express from 'express';
import cors from 'cors';
import './firebase';
import { AlarmRequestType, WaterRequestType, RequestHistory } from '@gbaranski/types';
import { isAuthenticated } from './auth';
import { waterMixerHandleRequest, waterMixerFetchEspDataInterval } from './watermixer';
import { getHistory, createHistory, getIpStr } from './helpers';
import { AlarmclockFetchEspDataInterval, AlarmclockHandleRequest } from './alarmclock';

if (!process.env.GBARANSKI) {
  throw new Error('missing env AUTH_KEY_GBARANSKI');
}
// export GOOGLE_APPLICATION_CREDENTIALS="/Users/gbaranski/code/firebase/firebase.json"
const httpPort = 8000;

const app = express();
const whitelist = ['https://control.gbaranski.com', 'http://localhost:3000', '*'];
app.use(cors({ origin: whitelist }));

app.use(express.json()); // for parsing application/json

// app.post('/getAlarmClock', (req, res) => {
//   console.log(req.body);
//   res.json(req.body);
// });

const deviceStatus = {
  alarmclock: false,
  watermixer: false,
  gate: false,
  garage: false,
};

const setAlarmclockStatus = (state: boolean): void => {
  deviceStatus.alarmclock = state;
};

const setWatermixerStatus = (state: boolean): void => {
  deviceStatus.watermixer = state;
};
setInterval(async () => {
  // remove async
  AlarmclockFetchEspDataInterval(setAlarmclockStatus);
}, 1000);

setInterval(async () => {
  // remove async
  waterMixerFetchEspDataInterval(setWatermixerStatus);
}, 1000);

app.listen(httpPort, () => console.log(`Example app listening at http://localhost:${httpPort}`));
