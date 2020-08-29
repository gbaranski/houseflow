import WebSocket from 'ws';
import { onConnection } from './app';

const { DEVICE_UID, DEVICE_SECRET, WS_URL } = process.env;

if (!DEVICE_UID || !DEVICE_SECRET || !WS_URL) {
  throw new Error('DEVICE_UID or DEVICE_SECRET are not defined in .env');
}

const websocket = new WebSocket(WS_URL, {
  headers: {
    uid: DEVICE_UID,
    secret: DEVICE_SECRET,
  },
});

websocket.on('open', () => onConnection(websocket));
