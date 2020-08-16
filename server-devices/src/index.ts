import { IncomingMessage } from 'http';
import WebSocket from 'ws';
import chalk from 'chalk';
import http from 'http';
import { convertToFirebaseDevice } from '@/services/firebase';
import { getIpStr } from '@/services/misc';
import {
  verifyDevice,
  validateConnection,
  assignDevice,
} from '@/services/websocket';

const requestListener: http.RequestListener = (req, res) => {
  res.writeHead(200);
  res.end('Hello from device zone');
};

const httpServer = http.createServer(requestListener);

export const wss: WebSocket.Server = new WebSocket.Server({
  server: httpServer,
  clientTracking: true,
  verifyClient: verifyDevice,
});

wss.on('connection', async (ws, req: IncomingMessage) => {
  try {
    const { uid, deviceName } = validateConnection(req);

    const firebaseDevice = await convertToFirebaseDevice(uid);

    assignDevice(ws, req, firebaseDevice);
    console.log(
      `New connection ${deviceName} IP:${getIpStr(req)} UID:${
        req.headers['uid']
      }`,
    );
  } catch (e) {
    console.error(`Error on connection! ${e.message}`);
    return;
  }
});

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
httpServer.listen(process.env.PORT, '0.0.0.0', () =>
  console.log(
    chalk.yellow(
      `Listening for websocket_devices connection at port ${process.env.PORT}`,
    ),
  ),
);
