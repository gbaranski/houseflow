import http from 'http';
import app from './app';
import chalk from 'chalk';

if (!process.env.PORT) throw new Error('Port not defined in .env');

const httpServer = http.createServer(app);

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
httpServer.listen(process.env.PORT, '0.0.0.0', () => {
  console.log(
    chalk.yellowBright(
      `Listening for http requests on port ${process.env.PORT}`,
    ),
  );
});
