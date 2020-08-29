import WebSocket from 'ws';
import chalk from 'chalk';
import http from 'http';
import { verifyDevice, onConnection } from '@/services/websocket';
import '@/services/redis_pub';
import '@/services/redis_sub';

const PORT = process.env.PORT_DEVICE;
if (!PORT) throw new Error('Port is not defined in .env');

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

wss.on('connection', onConnection);
wss.on('close', () => console.log("Connection closed"))
wss.on('error', () => console.log("Connection error"));

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
httpServer.listen(PORT, '0.0.0.0', () =>
  console.log(
    chalk.yellow(`Listening for websocket_devices connection at port ${PORT}`),
  ),
);
