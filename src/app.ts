import express from 'express';
import cors from 'cors';
import routes from './routes';
import { isAuthenticated } from './auth';
import { alarmclockInterval } from './routes/alarmclock/interval';
import { watermixerInterval } from './routes/watermixer/interval';
import { CORS_WHITELIST } from './config';

const URL_WITHOUT_LOGIN = ['/api/login'];

const app = express();
app.use(cors({ origin: CORS_WHITELIST }));
app.use(express.json()); // for parsing application/json

app.use((req, res, next): void => {
  console.log('attempt');
  if (URL_WITHOUT_LOGIN.includes(req.url)) {
    console.log('Bypassing login');
    next();
    return;
  }
  const username = req.header('username');
  const password = req.header('password');
  if (!isAuthenticated(username, password)) {
    res.sendStatus(401);
    console.log('Unathenticated');
    return;
  } else {
    console.log('authenticated');
    next();
  }
});

app.use('/', routes);

setInterval(alarmclockInterval, 1000);
setInterval(watermixerInterval, 1000);

export default app;
