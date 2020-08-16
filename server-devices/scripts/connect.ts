import WebSocket from 'ws';
import fetch from 'node-fetch';
import { Device, Alarmclock } from '@gbaranski/types';

(async () => {
  const URL = 'ws://localhost:8002';
  const req = await fetch('http://localhost:8000/api/getDeviceToken', {
    headers: {
      uid: '93ce1298-0782-42b7-916e-9a7d01e1ea8d',
      deviceType: 'ALARMCLOCK',
      secret: '873e5133-c7d1-4a7a-bd22-b98793417917',
    },
  });
  const ws = new WebSocket(URL, {
    headers: {
      uid: '93ce1298-0782-42b7-916e-9a7d01e1ea8d',
      deviceType: 'ALARMCLOCK',
      secret: '873e5133-c7d1-4a7a-bd22-b98793417917',
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
