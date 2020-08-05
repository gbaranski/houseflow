import { IncomingMessage } from 'http';
import WebSocket from 'ws';
import { logSocketConnection } from '@/cli';
import chalk from 'chalk';
import { verifyClient } from '@/auth';
import http from 'http';
import { logError } from '@/cli';
import WebSocketClient, { currentClients } from '@/client';

const httpServer = http.createServer();

export const wss: WebSocket.Server = new WebSocket.Server({
  server: httpServer,
  clientTracking: true,
  verifyClient,
});

wss.on('connection', (ws, req: IncomingMessage) => {
  const clientName = req.headers['username'];
  if (!clientName || clientName instanceof Array) {
    console.error('Error during recognizing client');
    ws.terminate();
    return;
  }
  logSocketConnection(req, clientName, 'client');

  const wsClient = new WebSocketClient(ws, clientName);
  setupWebsocketHandlers(ws, wsClient);
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
  currentClients.push(client);
  const pingInterval = setInterval(() => {
    if (!client.status) {
      return ws.terminate();
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
