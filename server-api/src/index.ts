import http from 'http';
import app from './app';
import chalk from 'chalk';

const PORT = 8000;
export const JWT_KEY = process.env.JWT_KEY as string;

if (!JWT_KEY) throw new Error('JWT_TOKEN is not defined');

const httpServer = http.createServer(app);

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
httpServer.listen(PORT, '0.0.0.0', () => {
  console.log(
    chalk.yellowBright(`Listening for http requests on port ${PORT}`),
  );
});
