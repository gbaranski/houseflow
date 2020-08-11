import WebSocket from 'ws';
import { v4 as uuidv4 } from 'uuid';
import {
  alarmclockSample,
  AlarmclockData,
  CurrentDevice,
  DeviceType,
} from '@gbaranski/types';
import Device from '..';
import { addTemperatureToDb } from '@/services/firebase';

export default class AlarmclockDevice extends Device<AlarmclockData> {
  private lastCheckedMinute: number = Number.MAX_SAFE_INTEGER;

  constructor(ws: WebSocket, currentDevice: CurrentDevice) {
    super(ws, alarmclockSample, DeviceType.ALARMCLOCK, currentDevice.uid);
    setInterval(() => {
      this.interval();
    }, 60000);
  }
  handleMessage(message: string): void {
    console.log(message);
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
