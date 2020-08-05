import { IncomingMessage } from 'http';
import WebSocket from 'ws';
import { logSocketConnection } from '@/cli';
import chalk from 'chalk';
import { verifyDevice } from '@/auth';
import http from 'http';
import { DeviceType, DevicesTypes } from '@gbaranski/types';
import { logSocketPingPong, logError } from '@/cli';
import WatermixerDevice from '@/devices/watermixer';
import Device, { AnyDeviceObject } from '@/devices';

const httpServer = http.createServer();

export const wss: WebSocket.Server = new WebSocket.Server({
  server: httpServer,
  clientTracking: true,
  verifyClient: verifyDevice,
});

wss.on('connection', (ws, req: IncomingMessage) => {
  const deviceName = req.headers['device'] as DevicesTypes;
  if (!deviceName) {
    console.error('Error during recognizing device');
    ws.terminate();
    return;
  }
  assignDevice(ws, DeviceType[deviceName]);
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

const assignDevice = (ws: WebSocket, deviceType: DeviceType) => {
  switch (deviceType) {
    case DeviceType.WATERMIXER:
      const watermixer = new WatermixerDevice(ws);
      setupWebsocketHandlers(ws, watermixer);
      break;
    case DeviceType.ALARMCLOCK:
      const alarmclock = new WatermixerDevice(ws);
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
    if (!device.deviceStatus) {
      return terminateConnection('Ping not received');
    }
    device.deviceStatus = false;
    ws.ping();
  }, 2000);

  ws.on('message', device.handleMessage);
  ws.on('pong', () => {
    device.deviceStatus = true;
    logSocketPingPong(device.deviceName, 'PONG', 'device');
  });
  ws.on('ping', () => {
    ws.pong();
    logSocketPingPong(device.deviceName, 'PING', 'device');
  });
  ws.on('error', err => {
    logError(err.message);
  });
  ws.on('close', (code, reason) => {
    terminateConnection(`Connection closed CODE: ${code} REASON: ${reason}`);
  });
}
