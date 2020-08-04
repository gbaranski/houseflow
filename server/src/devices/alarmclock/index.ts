import WebSocket from 'ws';
import { v4 as uuidv4 } from 'uuid';
import { alarmclockSample, AlarmclockData } from '@gbaranski/types';
import Device from '..';
import { addTemperatureToDb } from '@/services/firebase';

export default class AlarmclockDevice extends Device<AlarmclockData> {
  private lastCheckedMinute: number = Number.MAX_SAFE_INTEGER;

  constructor(ws: WebSocket) {
    super(ws, alarmclockSample, 'ALARMCLOCK', uuidv4());
    setInterval(() => {
      this.interval();
    }, 60000);
  }

  private interval(): void {
    if (
      new Date().getMinutes() === 0 &&
      new Date().getMinutes() !== this.lastCheckedMinute
    ) {
      this.lastCheckedMinute = new Date().getMinutes();
      addTemperatureToDb({
        unixTime: new Date().getTime(),
        temperature: super.deviceData.sensor.temperature,
      });
    }
  }
}
