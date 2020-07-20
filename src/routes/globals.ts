/* eslint-disable no-console */
import { devicesSample, Devices } from '@gbaranski/types';
import WebSocket from 'ws';
import { httpServer } from '../';
import { verifyClient } from '../auth';
import { IncomingMessage } from 'http';
import { setupWebsocketHandlers } from '../helpers';
import { setAlarmclockState, getAlarmclockState } from './alarmclock';
import { setWatermixerState, getWatermixerState } from './watermixer';
import { logSocketConnection } from '../cli';

export const devices: Devices = {
  ...devicesSample,
};

export let wss: WebSocket.Server;

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

export default function initializeWebsocket(): void {
  wss = new WebSocket.Server({
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
}
