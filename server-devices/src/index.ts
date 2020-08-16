import WebSocket from 'ws';
import chalk from 'chalk';
import http from 'http';
import { verifyDevice, onConnection } from '@/services/websocket';
import mongoose from 'mongoose';

if (!process.env.MONGODB_URI)
  throw new Error('MONGODB_URI is not defined in .env');

mongoose.connect(process.env.MONGODB_URI, {
  useNewUrlParser: true,
  useUnifiedTopology: true,
});

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

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
httpServer.listen(process.env.PORT, '0.0.0.0', () =>
  console.log(
    chalk.yellow(
      `Listening for websocket_devices connection at port ${process.env.PORT}`,
    ),
  ),
);
