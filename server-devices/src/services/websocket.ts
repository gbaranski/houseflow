import jwt from 'jsonwebtoken';
import { VerifyInfo, VerifyCallback } from '@/types';
import { Device as DeviceType, Watermixer, Alarmclock } from '@gbaranski/types';
import { IncomingMessage } from 'http';
import WebSocket from 'ws';
import { getIpStr } from '@/services/misc';
import WatermixerDevice from '@/devices/watermixer';
import AlarmclockDevice from '@/devices/alarmclock';
import Device, { AnyDeviceObject } from '@/devices';
import { convertToFirebaseDevice } from '@/services/firebase';

export const verifyDevice = (
  info: VerifyInfo,
  callback: VerifyCallback,
): void => {
  console.log(
    `Websocket attempt ${info.req.headers['devicetype']} UID: ${info.req.headers['uid']}`,
  );
  const token = info.req.headers.token || '';
  if (!token) {
    console.log(`JWT Token missing UID:${info.req.headers['uid']}`);
    callback(false, 401, 'Unauthorized');
    return;
  }

  if (!process.env.JWT_KEY) throw new Error('Missing process.env.JWT_KEY');
  jwt.verify(token as string, process.env.JWT_KEY, (err, decoded) => {
    if (!decoded) {
      console.log(`Decoded data is missing, UID:${info.req.headers['uid']}`);
      callback(false, 400, 'Missing decoded username');
      return;
    }
    if (err) {
      callback(false, 401, 'Unauthorized');
      console.log(`Invalid token UID:${info.req.headers['uid']}`);
    } else {
      info.req.headers.device = (decoded as { device: string }).device;
      callback(true);
    }
  });
};

export const validateConnection = (
  req: IncomingMessage,
): { uid: string; deviceName: string } => {
  const deviceName = req.headers['devicetype'] as DeviceType.DeviceType;
  const uid = req.headers['uid'];
  const secret = req.headers['secret'];
  if (!uid || !secret || uid instanceof Array || secret instanceof Array)
    throw new Error('Missing or invalid uid/secret');

  if (!deviceName) {
    throw new Error('No DeviceName preset');
  }
  return {
    uid,
    deviceName,
  };
};

export const assignDevice = async (
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
  const terminateConnection = (reason: string) => {
    device.terminateConnection(reason);
    clearInterval(pingInterval);
  };

  const pingInterval = setInterval(() => {
    if (!device.status) {
      return terminateConnection('Ping not received');
    }
    device.status = false;
    ws.ping();
  }, 2000);

  ws.on('message', (message) => device.handleMessage(message));
  ws.on('pong', () => {
    device.status = true;
  });
  ws.on('ping', () => {
    ws.pong();
  });
  ws.on('error', (err) => {
    console.log();
    console.log(err.message);
    terminateConnection(`Connection error UID: ${device.firebaseDevice.uid}`);
  });
  ws.on('close', (code, reason) => {
    terminateConnection(`Connection closed CODE: ${code} REASON: ${reason}`);
  });
}

export const onConnection = async (ws: WebSocket, req: IncomingMessage) => {
  try {
    const { uid, deviceName } = validateConnection(req);

    const firebaseDevice = await convertToFirebaseDevice(uid);

    assignDevice(ws, req, firebaseDevice);
    console.log(
      `New connection ${deviceName} IP:${getIpStr(req)} UID:${
        req.headers['uid']
      }`,
    );
  } catch (e) {
    console.error(`Error on connection! ${e.message}`);
    return;
  }
};
