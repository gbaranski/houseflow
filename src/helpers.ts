/* eslint-disable no-console */
import express from 'express';
import WebSocket from 'ws';
import geoip from 'geoip-lite';
import { logPingPong, logError } from './cli';

export function getIpStr(req: express.Request): string {
  return req.get('X-Forwarded-For') || req.get('X-Real-IP ') || req.ip;
}

export function getCountryStr(ip: string): string {
  const geo = geoip.lookup(ip);
  return geo ? geo.country : 'unknown';
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
    logPingPong(name, false);
  });
  ws.on('ping', () => {
    ws.ping();
    logPingPong(name, true);
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
