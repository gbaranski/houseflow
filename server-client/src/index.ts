import WebSocket from 'ws';
import chalk from 'chalk';
import { verifyClient, onConnection } from '@/services/websocket';
import http from 'http';
import '@/services/redis_sub';
import '@/services/redis_pub';

const PORT = process.env.PORT_CLIENT;
if (!PORT) throw new Error('Port is not defined in .env');

const requestListener: http.RequestListener = (req, res) => {
  res.writeHead(200);
  res.end('Hello from client zone');
};

const httpServer = http.createServer(requestListener);

export const wss: WebSocket.Server = new WebSocket.Server({
  server: httpServer,
  clientTracking: true,
  verifyClient,
});

wss.on('connection', onConnection);

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
httpServer.listen(PORT, '0.0.0.0', () =>
  console.log(
    chalk.yellow(`Listening for websocket_clients connection at port ${PORT}`),
  ),
);
