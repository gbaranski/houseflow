/* eslint-disable no-console */
import { devicesSample, Devices } from '@gbaranski/types';
import WebSocket from 'ws';
import https from 'https';
import fs from 'fs';
import { verifyClient } from '@/auth';
import { IncomingMessage } from 'http';
import { setupWebsocketHandlers } from '@/helpers';
import { setAlarmclockState, getAlarmclockState } from './alarmclock';
import { setWatermixerState, getWatermixerState } from './watermixer';
import { logSocketConnection } from '@/cli';
import chalk from 'chalk';

export const devices: Devices = {
  ...devicesSample,
};

if (!process.env.SSL_CERT_PATH || !process.env.SSL_KEY_PATH) {
  throw new Error('Missing SSL config in dotenv');
}
const httpServer = https.createServer({
  cert: fs.readFileSync(process.env.SSL_CERT_PATH),
  key: fs.readFileSync(process.env.SSL_KEY_PATH),
});

export const wss: WebSocket.Server = new WebSocket.Server({
  server: httpServer,
  clientTracking: true,
  verifyClient,
});

wss.on('connection', (ws, req: IncomingMessage) => {
  const deviceName = req.headers.device;
  if (!deviceName) {
    console.error('Error during recognizing device');
    ws.terminate();
  }
  assignDeviceToStatus(ws, req, deviceName as string);
  logSocketConnection(req, deviceName || '');
});

// @ts-ignore
httpServer.listen(process.env.WSS_PORT, '0.0.0.0', () =>
  console.log(
    chalk.yellow(
      `Listening for websocket connection at port ${process.env.WSS_PORT}`,
    ),
  ),
);

export const getDeviceStatus = (): {
  alarmclock: boolean;
  watermixer: boolean;
} => {
  return {
    alarmclock: devices.alarmclock.status,
    watermixer: devices.watermixer.status,
  };
};

const assignDeviceToStatus = (
  ws: WebSocket,
  req: IncomingMessage,
  deviceName: string,
) => {
  switch (deviceName) {
    case 'ALARMCLOCK':
      devices.alarmclock = {
        ...devices.alarmclock,
        ws,
        req,
      };
      setupWebsocketHandlers(
        ws,
        setAlarmclockState,
        getAlarmclockState,
        'alarmclock',
      );
      break;
    case 'WATERMIXER':
      devices.watermixer = {
        ...devices.watermixer,
        ws,
        req,
      };
      setupWebsocketHandlers(
        ws,
        setWatermixerState,
        getWatermixerState,
        'watermixer',
      );
      break;
  }
};

export default function initializeWebsocket(server: https.Server): void {}
