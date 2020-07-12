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
  verifyClient: (info, cb) => {
    if (!process.env.JWT_KEY) throw new Error('Missing process.env.JWT_KEY');

    const token = info.req.headers.token || '';
    if (!token) {
      cb(false, 401, 'Unauthorized');
    } else {
      jwt.verify(token as string, process.env.JWT_KEY, (err, decoded) => {
        if (err) {
          cb(false, 401, 'Unauthorized');
        } else {
          console.log('Decoded', decoded);
          cb(true);
        }
      });
    }
  },
});

wss.on('connection', function connection(ws) {
  ws.on('message', function incoming(message) {
    console.log('received: %s', message);
  });

  ws.send('something');
});

httpServer.listen(process.env.PORT, () => {
  console.log('\x1b[33m%s\x1b[0m', `Listening on port ${process.env.PORT}`);
});

export default httpServer;
