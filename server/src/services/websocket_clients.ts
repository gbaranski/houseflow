import { IncomingMessage } from 'http';
import WebSocket from 'ws';
import { logSocketConnection } from '@/cli';
import chalk from 'chalk';
import { verifyClient } from '@/auth';
import http from 'http';
import { logError } from '@/cli';
import WebSocketClient from '@/client';
import { decodeClientToken } from './firebase';

const httpServer = http.createServer();

export const wss: WebSocket.Server = new WebSocket.Server({
  server: httpServer,
  clientTracking: true,
  verifyClient,
});

wss.on('connection', (ws, req: IncomingMessage) => {
  console.log(req.headers);
  const rawToken = req.headers['sec-websocket-protocol'];
  if (!rawToken || rawToken instanceof Array) {
    console.error('Missing or invalid token');
    ws.terminate();
    return;
  }
  decodeClientToken(rawToken)
    .then(client => {
      logSocketConnection(req, client.uid, 'client');

      const wsClient = new WebSocketClient(ws, client.uid);
      setupWebsocketHandlers(ws, wsClient);
    })
    .catch(e => {
      console.error(e);
      ws.terminate();
    });
});

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
httpServer.listen(process.env.WS_CLIENT_PORT, '0.0.0.0', () =>
  console.log(
    chalk.yellow(
      `Listening for websocket_clients connection at port ${process.env.WS_CLIENT_PORT}`,
    ),
  ),
);

export function setupWebsocketHandlers(
  ws: WebSocket,
  client: WebSocketClient,
): void {
  WebSocketClient.addNewClient(client);

  const terminateConnection = (reason: string) => {
    client.terminateConnection(reason);
    WebSocketClient.removeClient(client);
    clearInterval(pingInterval);
  };

  const pingInterval = setInterval(() => {
    if (!client.status) {
      return terminateConnection('Ping not received');
    }
    client.status = false;
    ws.ping();
  }, 2000);
  ws.on('message', client.handleMessage);
  ws.on('pong', () => {
    client.status = true;
  });
  ws.on('ping', () => {
    ws.pong();
  });
  ws.on('error', err => {
    logError(err.message);
  });
  ws.on('close', (code, reason) => {
    logError(`CODE: ${code} \nREASON:${reason}`);
    clearInterval(pingInterval);
    ws.terminate();
  });
}
