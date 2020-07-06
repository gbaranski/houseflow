/* eslint-disable no-console */
import express from 'express';
import cors from 'cors';
import './firebase';
import { isAuthenticated } from './auth';
import { alarmclockInterval } from './routes/alarmclock/interval';
import { watermixerInterval } from './routes/watermixer/interval';
import { getProcessing } from './routes/globals';

if (!process.env.GBARANSKI) {
  throw new Error('missing env AUTH_KEY_GBARANSKI');
}
// export GOOGLE_APPLICATION_CREDENTIALS="/Users/gbaranski/code/firebase/firebase.json"
const httpPort = 8000;

const app = express();
const whitelist = [
  'https://control.gbaranski.com',
  'http://localhost:3000',
  '*',
];

app.use(cors({ origin: whitelist }));
app.use(express.json()); // for parsing application/json

app.use((req, res, next): void => {
  if (req.method !== 'POST') {
    next();
    return;
  }
  const username = req.header('username');
  const password = req.header('password');
  if (!isAuthenticated(username, password)) {
    res.status(401).end();
  } else {
    next();
  }
});

setInterval(alarmclockInterval, 1000);
setInterval(watermixerInterval, 1000);

app.listen(httpPort, (): void =>
  console.log(`API-Server listening at http://localhost:${httpPort}`),
);
