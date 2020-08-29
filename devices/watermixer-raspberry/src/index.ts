import WebSocket from 'ws';

const { DEVICE_UID, DEVICE_SECRET, WS_URL } = process.env;

if (!DEVICE_UID || !DEVICE_SECRET || !WS_URL) {
  throw new Error('DEVICE_UID or DEVICE_SECRET are not defined in .env');
}

const websocket = new WebSocket(WS_URL);
websocket.on('open', () => console.log('COnnection opened'));
