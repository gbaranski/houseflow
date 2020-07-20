import { AlarmclockData } from '@gbaranski/types';
import { devices } from '../globals';

const SECONDS_IN_HOUR = 3600;
let secondsPassed = SECONDS_IN_HOUR;

export function alarmclockInterval(): void {
  secondsPassed += 1;
  if (!devices.alarmclock.ws) {
    handleTempArray();
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
  if (secondsPassed >= SECONDS_IN_HOUR) {
    devices.alarmclock.tempArray.shift();
    devices.alarmclock.tempArray.push({
      temp: devices.alarmclock.data.temperature,
      unixTime: new Date().getTime(),
    });
    secondsPassed = 0;
  }
};
