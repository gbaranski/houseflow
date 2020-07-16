/* eslint-disable no-console */
import { DeviceStatus } from '@gbaranski/types';
import WebSocket from 'ws';
import { httpServer } from '../';
import { verifyClient } from '../auth';

export let wss: WebSocket.Server;

const deviceStatusPattern = {
  alarmclock: false,
  watermixer: false,
  gate: false,
  garage: false,
};

let deviceStatus: DeviceStatus = {
  ...deviceStatusPattern,
};

let isProcessing: DeviceStatus = {
  ...deviceStatusPattern,
};

export function setDeviceStatus(newStatus: DeviceStatus): void {
  deviceStatus = newStatus;
}

export function getDeviceStatus(): DeviceStatus {
  return deviceStatus;
}

export function setProcessing(newIsProcessing: DeviceStatus): void {
  isProcessing = newIsProcessing;
}

export function getProcessing(): DeviceStatus {
  return isProcessing;
}

const setDeviceStatusByString = (device: string, state: boolean) => {
  switch (device) {
    case 'ALARMCLOCK':
      deviceStatus.alarmclock = state;
      break;
    case 'WATERMIXER':
      deviceStatus.watermixer = state;
      break;
    default:
      console.error('Couldnt find device, sending false');
      break;
  }
};

const getDeviceStatusByString = (device: string) => {
  switch (device) {
    case 'ALARMCLOCK':
      return deviceStatus.alarmclock;
    case 'WATERMIXER':
      return deviceStatus.watermixer;
    default:
      console.error('Couldnt find device, sending false');
      return false;
  }
};

export default function initializeWebsocket(): void {
  setInterval(() => {
    console.dir(getDeviceStatus());
  }, 1000);

  wss = new WebSocket.Server({
    server: httpServer,
    clientTracking: true,
    verifyClient,
  });

  wss.on('connection', (ws, req) => {
    const deviceName = req.headers.device as string;

    if (!deviceName) {
      console.log("Couldn't recognize device, will terminate in a moment");
    }
    console.log(
      `Websocket Connection device: ${deviceName} from IP: ${req.socket.remoteAddress} at PORT: ${req.socket.remotePort}`,
    );

    setDeviceStatusByString(deviceName, true);

    const pingInterval = setInterval(() => {
      if (getDeviceStatusByString(deviceName) === false) return ws.terminate();
      setDeviceStatusByString(deviceName, false);
      ws.ping();
    }, 30000);

    ws.on('pong', () => {
      setDeviceStatusByString(deviceName, true);
      console.log(`Recieved pong from ${deviceName}`);
    });
    ws.on('ping', () => {
      ws.ping();
      console.log(`Recieved ping from ${deviceName}`);
    });
    ws.on('message', message => {
      console.log('received: %s', message);
    });
    ws.on('error', err => {
      console.log('Error occured', err);
      clearInterval(pingInterval);
    });
    ws.on('close', () => clearInterval(pingInterval));
    ws.send('something');
  });
  wss.on('error', cb => {
    console.log('Error occured', cb.message);
  });
}
