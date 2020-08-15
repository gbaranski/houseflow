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
import { Device as DeviceType, Watermixer, Alarmclock } from '@gbaranski/types';
import { logError } from '@/cli';
import WatermixerDevice from '@/devices/watermixer';
import Device, { AnyDeviceObject } from '@/devices';
import AlarmclockDevice from '@/devices/alarmclock';
import { convertToFirebaseDevice } from './firebase';
import { VerifyInfo, VerifyCallback } from '@/auth';
import { getIpStr } from './resolveip';

const requestListener: http.RequestListener = (req, res) => {
  res.writeHead(200);
  res.end('Hello from device zone');
};

const httpServer = http.createServer(requestListener);

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

wss.on('connection', async (ws, req: IncomingMessage) => {
  const deviceName = req.headers['devicetype'] as DeviceType.DeviceType;
  const uid = req.headers['uid'];
  const secret = req.headers['secret'];
  if (!uid || !secret || uid instanceof Array || secret instanceof Array)
    throw new Error('Missing or invalid uid/secret');

  if (!deviceName) {
    console.error('Error during recognizing device');
    ws.terminate();
    return;
  }

  const firebaseDevice = await convertToFirebaseDevice(uid);
  assignDevice(ws, req, firebaseDevice);
  logSocketConnection(req, 'device', deviceName);
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
  req: IncomingMessage,
  firebaseDevice: DeviceType.FirebaseDevice,
) => {
  const ip = getIpStr(req);
  switch (firebaseDevice.type) {
    case 'WATERMIXER':
      const watermixer = new WatermixerDevice(ws, firebaseDevice, {
        ...firebaseDevice,
        data: Watermixer.SAMPLE,
        ip,
      });
      setupWebsocketHandlers(ws, watermixer);
      break;
    case 'ALARMCLOCK':
      const alarmclock = new AlarmclockDevice(ws, firebaseDevice, {
        ...firebaseDevice,
        data: Alarmclock.SAMPLE,
        ip,
      });
      setupWebsocketHandlers(ws, alarmclock);
      break;
    default:
      console.log(
        `Error recognizing device with type ${firebaseDevice.type}!, terminating`,
      );
      ws.terminate();
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

  ws.on('message', message => device.handleMessage(message));
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
