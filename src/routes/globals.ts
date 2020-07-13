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

export default function initializeWebsocket(): void {
  wss = new WebSocket.Server({
    server: httpServer,
    clientTracking: true,
    verifyClient,
  });

  wss.on('connection', function connection(ws, req) {
    console.log(
      `Websocket Connection device: ${req.headers.device} from IP: ${req.socket.remoteAddress} at PORT: ${req.socket.remotePort}`,
    );
    let i = 0;
    setInterval(() => {
      ws.send('Interval: ' + i);
      i++;
    }, 1000);

    ws.on('message', function incoming(message) {
      console.log('received: %s', message);
    });

    ws.send('something');
  });
}
