/* eslint-disable no-console */
import http from 'http';
import jwt from 'jsonwebtoken';
import app from './app';
import WebSocket from 'ws';
import './firebase';

if (!process.env.GBARANSKI) {
  throw new Error('missing env AUTH_KEY_GBARANSKI');
}
if (!process.env.JWT_KEY) {
  throw new Error('Missing process.env.JWT_KEY');
}

const httpServer = http.createServer(app);

const wss = new WebSocket.Server({
  server: httpServer,
  clientTracking: true,
  verifyClient: (info, cb) => {
    if (!process.env.JWT_KEY) throw new Error('Missing process.env.JWT_KEY');
    console.log(info.req.headers.token || undefined);
    const token = info.req.headers.token || '';
    if (!token) {
      console.log('client doesnt have token');
      cb(false, 401, 'Unauthorized');
    } else {
      jwt.verify(token as string, process.env.JWT_KEY, (err, decoded) => {
        if (!decoded) {
          console.log('Missing decoded username at JWT Token');
          cb(false, 400, 'Missing decoded username');
          return;
        }
        const decodedDeviceName = decoded.device;

        if (err) {
          cb(false, 401, 'Unauthorized');
          console.log('client has invalid token');
        } else {
          info.req.headers.device = decodedDeviceName;
          cb(true);
        }
      });
    }
  },
});

wss.on('connection', function connection(ws, req) {
  console.log(
    `Websocket Connection device: ${req.headers.device} from IP: ${req.socket.remoteAddress} at PORT: ${req.socket.remotePort}`,
  );
  let i = 0;
  setInterval(() => {
    ws.send('Interval: ' + i);
    i++;
  }, 1000);

  ws.on('message', function incoming(message) {
    console.log('received: %s', message);
  });

  ws.send('something');
});

httpServer.listen(process.env.PORT, () => {
  console.log('\x1b[33m%s\x1b[0m', `Listening on port ${process.env.PORT}`);
});

export default wss;
