import { VerifyInfo, VerifyCallback } from '@/types';
import { decodeClientToken, convertToFirebaseUser } from '@/services/firebase';
import WebSocket from 'ws';
import { IncomingMessage } from 'http';
import { getIpStr } from '@/services/misc';
import WebSocketClient from '@/client';
import { Client } from '@gbaranski/types';

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

export const onConnection = async (ws: WebSocket, req: IncomingMessage) => {
  const rawToken = req.headers['sec-websocket-protocol'];
  if (!rawToken || rawToken instanceof Array) {
    console.error('Missing or invalid token');
    ws.terminate();
    return;
  }

  try {
    const decodedClientId = decodeClientToken(rawToken);
    console.log(
      `New connection IP:${getIpStr(req)} UID:${(await decodedClientId).uid}`,
    );
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
};
