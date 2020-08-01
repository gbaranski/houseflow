import { IncomingMessage } from 'http';
import WebSocket from 'ws';
import { logSocketConnection } from '@/cli';
import chalk from 'chalk';
import fs from 'fs';
import { verifyClient } from '@/auth';
import https from 'https';
import { devices } from '@/routes/globals';
import { setupWebsocketHandlers } from '@/helpers';
import { setAlarmclockState, getAlarmclockState } from '@/routes/alarmclock';
import { setWatermixerState, getWatermixerState } from '@/routes/watermixer';

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
