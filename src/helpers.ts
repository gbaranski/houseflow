/* eslint-disable no-console */
import express from 'express';
import fetch, { Headers } from 'node-fetch';
import WebSocket from 'ws';

export function getIpStr(req: express.Request): string {
  return String(req.get('cf-connecting-ip') || req.connection.remoteAddress);
}

export function getCountryStr(req: express.Request): string {
  return String(req.header('cf-ipcountry'));
}

export async function fetchURL(
  url: string,
  path: string,
  headers?: Headers,
): Promise<number> {
  let statusCode = 0;
  await fetch(`${url}${path}`, {
    method: 'POST',
    headers: headers ? headers : new Headers(),
  })
    .then(_data => {
      console.log('Success:', _data.status);
      statusCode = _data.status;
    })
    .catch(() => {
      console.error(`Error while fetching ${path}`);
      statusCode = 503;
    });
  return statusCode;
}

export function setupWebsocketHandlers(
  ws: WebSocket,
  setState: (state: boolean) => void,
  getState: () => boolean,
  name: string,
): void {
  const pingInterval = setInterval(() => {
    if (!getState()) return ws.terminate();
    setState(false);
    ws.ping();
  }, 10000);
  ws.on('pong', () => {
    setState(true);
    console.log(`Recieved pong from ${name}`);
  });
  ws.on('ping', () => {
    ws.ping();
    console.log(`Recieved ping from ${name}`);
  });
  ws.on('error', err => {
    console.log('Error occured', err.message);
  });
  ws.on('close', (code, reason) => {
    console.log(`CODE: ${code} \nREASON:${reason}`);
    clearInterval(pingInterval);
    ws.terminate();
  });
}
