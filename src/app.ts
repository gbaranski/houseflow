import express from 'express';
import cors from 'cors';
import routes from './routes';
import { isAuthenticated } from './auth';
import { CORS_WHITELIST, LOGIN_WHITELIST_URL } from './config';
import { alarmclockInterval } from './routes/alarmclock/interval';
import { watermixerInterval } from './routes/watermixer/interval';
import { logRequest } from './cli';

const app = express();
app.use(cors({ origin: CORS_WHITELIST }));
app.use(express.json()); // for parsing application/json

app.use((req, res, next): void => {
  logRequest(req, res);
  if (LOGIN_WHITELIST_URL.includes(req.url)) {
    next();
    return;
  }
  isAuthenticated(req, res, next);
});
app.use(
  (
    err: { message: string },
    req: express.Request,
    res: express.Response,
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    _next: express.NextFunction,
  ): void => {
    res.status(401).send(err.message);
  },
);

app.use('/', routes);

console.log('\x1b[36m%s\x1b[0m', 'Starting IoT data fetch interval');
setInterval(alarmclockInterval, 1000);
setInterval(watermixerInterval, 500);

export default app;
