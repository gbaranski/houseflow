import { IncomingMessage } from 'http';
import WebSocket from 'ws';
import { logSocketConnection } from '@/cli';
import chalk from 'chalk';
import { VerifyInfo, VerifyCallback } from '@/auth';
import http from 'http';
import WebSocketClient from '@/client';
import { decodeClientToken, convertToFirebaseUser } from './firebase';
import { Client } from '@gbaranski/types';
import { getIpStr } from './resolveip';

const requestListener: http.RequestListener = (req, res) => {
  res.writeHead(200);
  res.end('Hello from client zone');
};

const httpServer = http.createServer(requestListener);

export const verifyClient = (
  info: VerifyInfo,
  callback: VerifyCallback,
): void => {
  const rawToken = info.req.headers['sec-websocket-protocol'];
  if (rawToken instanceof Array || !rawToken) {
    callback(false, 400, 'Invalid token headers');
    return;
  }
  try {
    decodeClientToken(rawToken);
  } catch (e) {
    console.log(e);
    callback(false, 401, 'Unauthorized');
    return;
  }
  callback(true);
};

export const wss: WebSocket.Server = new WebSocket.Server({
  server: httpServer,
  clientTracking: true,
  verifyClient,
});

wss.on('connection', async (ws, req: IncomingMessage) => {
  const rawToken = req.headers['sec-websocket-protocol'];
  if (!rawToken || rawToken instanceof Array) {
    console.error('Missing or invalid token');
    ws.terminate();
    return;
  }

  logSocketConnection(req, 'client');
  try {
    const decodedClientId = decodeClientToken(rawToken);
    const firebaseUser = await convertToFirebaseUser(
      (await decodedClientId).uid,
    );
    const activeUser: Client.ActiveUser = {
      ...firebaseUser,
      ip: getIpStr(req),
    };
    new WebSocketClient(ws, firebaseUser, activeUser);
  } catch (e) {
    console.error(e);
    ws.terminate();
  }
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
