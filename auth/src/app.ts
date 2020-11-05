import express from 'express';
import cors from 'cors';
import routes from '@/routes/routes';
import morgan from 'morgan';
import chalk from 'chalk';

const CORS_WHITELIST = ['https://houseflow.gbaranski.com', '*'];

export const app = express();
app.use(cors({ origin: CORS_WHITELIST }));
morgan.token('code', (req: express.Request, res: express.Response) => {
  const code = res.statusCode;
  if (code === 400 || code === 401 || code === 403) {
    return chalk.red.bold(code);
  } else {
    return chalk.green.bold(code);
  }
});

app.use(
  morgan((tokens, req, res) => {
    return [
      chalk.cyan(tokens.method(req, res)),
      chalk.cyan(tokens.url(req, res)),
      tokens.code(req, res),
      chalk.dim('-'),
      chalk.dim.bold(tokens['response-time'](req, res)),
      chalk.dim.bold('ms'),
    ].join(' ');
  }),
);
app.use(express.json()); // for parsing application/json

app.use('/', routes);

export default app;
