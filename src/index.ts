/* eslint-disable no-console */
import http from 'http';
import app from './app';
import './firebase';

if (!process.env.GBARANSKI) {
  throw new Error('missing env AUTH_KEY_GBARANSKI');
}
const httpServer = http.createServer(app);
httpServer.listen(process.env.PORT);
console.log('\x1b[33m%s\x1b[0m', `Listening on port ${process.env.PORT}`);
