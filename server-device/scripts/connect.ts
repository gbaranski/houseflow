import WebSocket from 'ws';
import fetch from 'node-fetch';
import { Device, Alarmclock } from '@gbaranski/types';

const deviceSecret = process.env.TEST_SECRET;
const deviceUid = process.env.TEST_UID;
const deviceType = process.env.TEST_TYPE;

(async () => {
  const URL = 'ws://localhost:8002';
  const req = await fetch('http://localhost:8000/api/getDeviceToken', {
    headers: {
      uid: deviceUid,
      deviceType: deviceType,
      secret: deviceSecret,
    },
  });
  const ws = new WebSocket(URL, {
    headers: {
      uid: deviceUid,
      deviceType: deviceType,
      secret: deviceSecret,
      token: await req.text(),
    },
  });
  ws.onopen = () => {
    console.log('Connection opened');
    setInterval(() => {
      const dataMsg: Device.ResponseDevice<Alarmclock.Data> = {
        ok: true,
        responseFor: 'GET_DATA',
        data: {
          alarmState: true,
          alarmTime: {
            hour: new Date().getHours(),
            minute: new Date().getMinutes(),
            second: new Date().getSeconds(),
          },
          sensor: {
            temperature: Math.random() * 10,
            humidity: Math.random(),
            heatIndex: Math.random(),
          },
        },
      };
      ws.send(JSON.stringify(dataMsg));
    }, 500);
  };
})();
