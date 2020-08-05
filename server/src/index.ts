/* eslint-disable no-console */
require('module-alias/register');
import http from 'http';
import app from '@/app';
import '@/services/firebase';
import '@/services/websocket_devices';
import '@/services/websocket_clients';
import '@/cli/index';
import chalk from 'chalk';

if (!process.env.GBARANSKI) {
  throw new Error('missing env AUTH_KEY_GBARANSKI');
}
if (!process.env.JWT_KEY) {
  throw new Error('Missing process.env.JWT_KEY');
}

const httpServer = http.createServer(app);

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
httpServer.listen(process.env.HTTP_PORT, '0.0.0.0', () => {
  console.log(
    chalk.yellowBright(
      `Listening for http requests on port ${process.env.HTTP_PORT}`,
    ),
  );
});
