import WebSocket from 'ws';
import { Device, Watermixer } from '@gbaranski/types';

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

websocket.on('open', () => {
  console.log('Connection opened')
  setInterval(() => {
    const request: Device.ResponseDevice<Watermixer.Data> = {
      responseFor: 'GET_DATA',
      ok: true,
      data: Watermixer.SAMPLE,
    };
    websocket.send(JSON.stringify(request));
  }, 500)
});
websocket.on('pong', function () { this.pong() })
