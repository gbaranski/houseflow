/* eslint-disable no-console */
import https from 'https';
import app from './app';
import fs from 'fs';
import './firebase';
import initializeWebsocket from './routes/globals';

if (!process.env.GBARANSKI) {
  throw new Error('missing env AUTH_KEY_GBARANSKI');
}
if (!process.env.JWT_KEY) {
  throw new Error('Missing process.env.JWT_KEY');
}

const httpServer = https.createServer(
  {
    cert: fs.readFileSync(process.env.SSL_CERT_PATH),
    key: fs.readFileSync(process.env.SSL_KEY_PATH),
  },
  app,
);

initializeWebsocket(httpServer);

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
httpServer.listen(process.env.PORT, '0.0.0.0', () => {
  console.log('\x1b[33m%s\x1b[0m', `Listening on port ${process.env.PORT}`);
});
