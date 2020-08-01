/* eslint-disable no-console */
require('module-alias/register');
import https from 'https';
import app from '@/app';
import fs from 'fs';
import '@/services/firebase';
import '@/cli/index';
import chalk from 'chalk';

if (!process.env.SSL_CERT_PATH || !process.env.SSL_KEY_PATH) {
  throw new Error('Missing SSL config in dotenv');
}

if (!process.env.GBARANSKI) {
  throw new Error('missing env AUTH_KEY_GBARANSKI');
}
if (!process.env.JWT_KEY) {
  throw new Error('Missing process.env.JWT_KEY');
}

const httpServer = https.createServer(
  {
    cert: fs.readFileSync(process.env.SSL_CERT_PATH as string),
    key: fs.readFileSync(process.env.SSL_KEY_PATH as string),
  },
  app,
);

// @ts-ignore
httpServer.listen(process.env.HTTPS_PORT, '0.0.0.0', () => {
  console.log(
    chalk.yellowBright(
      `Listening for http requests on port ${process.env.HTTPS_PORT}`,
    ),
  );
});
