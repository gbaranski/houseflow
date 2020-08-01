import { IncomingMessage } from 'http';
import WebSocket from 'ws';
import { logSocketConnection } from '@/cli';
import chalk from 'chalk';
import { verifyClient } from '@/auth';
import http from 'http';
import { devices } from '@/routes/globals';
import { setupWebsocketHandlers } from '@/helpers';
import { setAlarmclockState, getAlarmclockState } from '@/routes/alarmclock';
import { setWatermixerState, getWatermixerState } from '@/routes/watermixer';

const httpServer = http.createServer();

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
