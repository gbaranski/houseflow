import { AlarmclockData } from '@gbaranski/types';
import { devices } from '../globals';
import { addTemperatureToDb } from '../../firebase';

export function alarmclockInterval(): void {
  if (!devices.alarmclock.ws) {
    return;
  }

  if (!devices.alarmclock.status) {
    handleTempArray();
    return;
  }
  devices.alarmclock.ws.send('GET_DATA');
  devices.alarmclock.ws.addEventListener(
    'message',
    (message: { data: string; type: string; target: WebSocket }) => {
      devices.alarmclock.data = JSON.parse(message.data) as AlarmclockData;
      handleTempArray();
    },
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    { once: true },
  );
}

const handleTempArray = () => {
  if (new Date().getMinutes() === 0) {
    addTemperatureToDb({
      unixTime: new Date().getTime(),
      temperature: devices.alarmclock.data.temperature,
    });
  }
};
