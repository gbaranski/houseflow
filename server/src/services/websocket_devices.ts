import { IncomingMessage } from 'http';
import WebSocket from 'ws';
import {
  logSocketConnection,
  logSocketAttempt,
  logMissing,
  logInvalid,
} from '@/cli';
import jwt from 'jsonwebtoken';
import chalk from 'chalk';
import http from 'http';
import { DeviceType, DevicesTypes } from '@gbaranski/types';
import { logError } from '@/cli';
import WatermixerDevice from '@/devices/watermixer';
import Device, { AnyDeviceObject } from '@/devices';
import AlarmclockDevice from '@/devices/alarmclock';
import { validateDevice } from './firebase';
import { VerifyInfo, VerifyCallback } from '@/auth';

const httpServer = http.createServer();

export const verifyDevice = (
  info: VerifyInfo,
  callback: VerifyCallback,
): void => {
  logSocketAttempt(
    info.req,
    info.req.headers['devicetype'] || 'unknown',
    'device',
  );
  if (!process.env.JWT_KEY) throw new Error('Missing process.env.JWT_KEY');
  const token = info.req.headers.token || '';
  if (!token) {
    logMissing('JWT token');
    callback(false, 401, 'Unauthorized');
    return;
  }

  jwt.verify(token as string, process.env.JWT_KEY, (err, decoded) => {
    if (!decoded) {
      logMissing('decoded username at JWT Token');
      callback(false, 400, 'Missing decoded username');
      return;
    }
    if (err) {
      callback(false, 401, 'Unauthorized');
      logInvalid('token');
    } else {
      info.req.headers.device = (decoded as { device: string }).device;
      callback(true);
    }
  });
};

export const wss: WebSocket.Server = new WebSocket.Server({
  server: httpServer,
  clientTracking: true,
  verifyClient: verifyDevice,
});

wss.on('connection', (ws, req: IncomingMessage) => {
  const deviceName = req.headers['devicetype'] as DevicesTypes;
  const uid = req.headers['uid'];
  const secret = req.headers['secret'];
  if (!uid || !secret || uid instanceof Array || secret instanceof Array)
    throw new Error('Missing or invalid uid/secret');

  if (!deviceName) {
    console.error('Error during recognizing device');
    ws.terminate();
    return;
  }
  assignDevice(ws, DeviceType[deviceName], uid, secret);
  logSocketConnection(req, deviceName, 'device');
});

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
httpServer.listen(process.env.WS_DEVICE_PORT, '0.0.0.0', () =>
  console.log(
    chalk.yellow(
      `Listening for websocket_devices connection at port ${process.env.WS_DEVICE_PORT}`,
    ),
  ),
);

export const getWssClients = (): Set<WebSocket> => {
  return wss.clients;
};

const assignDevice = async (
  ws: WebSocket,
  deviceType: DeviceType,
  uid: string,
  secret: string,
) => {
  const currentDevice = await validateDevice(deviceType, uid, secret);
  switch (deviceType) {
    case DeviceType.WATERMIXER:
      const watermixer = new WatermixerDevice(ws, currentDevice);
      setupWebsocketHandlers(ws, watermixer);
      break;
    case DeviceType.ALARMCLOCK:
      const alarmclock = new AlarmclockDevice(ws, currentDevice);
      setupWebsocketHandlers(ws, alarmclock);
      break;
  }
};

export function setupWebsocketHandlers(
  ws: WebSocket,
  device: AnyDeviceObject,
): void {
  Device.addNewDevice(device);

  const terminateConnection = (reason: string) => {
    device.terminateConnection(reason);
    Device.removeDevice(device);
    clearInterval(pingInterval);
  };

  const pingInterval = setInterval(() => {
    if (!device.status) {
      return terminateConnection('Ping not received');
    }
    device.status = false;
    ws.ping();
  }, 2000);

  ws.on('message', device.handleMessage);
  ws.on('pong', () => {
    device.status = true;
  });
  ws.on('ping', () => {
    ws.pong();
  });
  ws.on('error', err => {
    logError(err.message);
  });
  ws.on('close', (code, reason) => {
    terminateConnection(`Connection closed CODE: ${code} REASON: ${reason}`);
  });
}
