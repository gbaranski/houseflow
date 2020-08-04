import { IncomingMessage } from 'http';
import WebSocket from 'ws';
import { logSocketConnection } from '@/cli';
import chalk from 'chalk';
import { verifyClient } from '@/auth';
import http from 'http';
import { DeviceType, DevicesTypes, ResponseDevice } from '@gbaranski/types';
import { logPingPong, logError } from '@/cli';
import WatermixerDevice from '@/devices/watermixer';
import { currentDevices, AnyDeviceObject } from '@/devices/globals';

const httpServer = http.createServer();

export const wss: WebSocket.Server = new WebSocket.Server({
  server: httpServer,
  clientTracking: true,
  verifyClient,
});

wss.on('connection', (ws, req: IncomingMessage) => {
  const deviceName = req.headers['device'] as DevicesTypes;
  if (!deviceName) {
    console.error('Error during recognizing device');
    ws.terminate();
    return;
  }
  assignDevice(ws, DeviceType[deviceName]);
  logSocketConnection(req, deviceName || '');
});

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
httpServer.listen(process.env.WS_PORT, '0.0.0.0', () =>
  console.log(
    chalk.yellow(
      `Listening for websocket connection at port ${process.env.WS_PORT}`,
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
  currentDevices.push(device);

  const killBrokenConnection = setInterval(() => {
    if (device.failedRequests > 3) {
      device.terminateConnection('Too many failed requests');
      currentDevices.filter((_device: AnyDeviceObject) => _device === device);
      clearInterval(killBrokenConnection);
    }
  }, 1000);

  const pingInterval = setInterval(() => {
    if (!device.deviceStatus) return ws.terminate();
    device.deviceStatus = false;
    ws.ping();
  }, 10000);

  ws.on('message', device.handleMessage);
  ws.on('pong', () => {
    device.deviceStatus = true;
    logPingPong(device.deviceName, false);
  });
  ws.on('ping', () => {
    ws.ping();
    logPingPong(device.deviceName, true);
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

export const validateSocketMessage = (message: WebSocket.Data): void => {
  if (
    message instanceof Buffer ||
    message instanceof ArrayBuffer ||
    message instanceof Array
  )
    throw new Error('Cannot handle Buffer type');
  const parsedResponse = JSON.parse(message) as ResponseDevice<undefined>;
  if (!parsedResponse.ok) throw new Error('Response is not okay');
};
