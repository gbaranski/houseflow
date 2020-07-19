import express from 'express';
import cors from 'cors';
import routes from './routes';
import { isAuthenticated } from './auth';
import { CORS_WHITELIST, LOGIN_WHITELIST_URL } from './config';
import { getIpStr, getCountryStr } from './helpers';
import { alarmclockInterval } from './routes/alarmclock/interval';
import { watermixerInterval } from './routes/watermixer/interval';

const app = express();
app.use(cors({ origin: CORS_WHITELIST }));
app.use(express.json()); // for parsing application/json

app.use((req, res, next): void => {
  if (LOGIN_WHITELIST_URL.includes(req.url)) {
    next();
    return;
  }
  console.log(
    // magenta color
    '\x1b[35m',
    `
    ==================== 
    IP: ${getIpStr(req)}                     \n
    User-agent: ${req.get('user-agent')}     \n
    URL: ${req.url}                          \n
    COUNTRY: ${getCountryStr(req)}           \n
    ==================== 
  `,
  );
  console.log('\x1b[0m');
  isAuthenticated(req, res, next);
  next();
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
