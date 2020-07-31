/* eslint-disable no-console */
import http from 'http';
import app from './app';
import './firebase';
import initializeWebsocket from './routes/globals';

if (!process.env.GBARANSKI) {
  throw new Error('missing env AUTH_KEY_GBARANSKI');
}
if (!process.env.JWT_KEY) {
  throw new Error('Missing process.env.JWT_KEY');
}

export const httpServer = http.createServer(app);

initializeWebsocket();

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
httpServer.listen(process.env.PORT, '0.0.0.0', () => {
  console.log('\x1b[33m%s\x1b[0m', `Listening on port ${process.env.PORT}`);
});
