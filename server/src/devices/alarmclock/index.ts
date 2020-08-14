import WebSocket from 'ws';
import Device from '..';
import { addTemperatureToDb } from '@/services/firebase';
import { Alarmclock, Device as DeviceType } from '@gbaranski/types';
import { validateDeviceMessage } from '@/helpers';

export default class AlarmclockDevice extends Device<Alarmclock.Data> {
  private lastCheckedMinute: number = Number.MAX_SAFE_INTEGER;

  constructor(ws: WebSocket, device: DeviceType.FirebaseDevice) {
    super(ws, Alarmclock.SAMPLE, 'ALARMCLOCK', device.uid);
    setInterval(() => {
      this.interval();
    }, 60000);
  }
  handleMessage(message: WebSocket.Data): void {
    validateDeviceMessage(message);
    const parsedResponse = JSON.parse(
      message as string,
    ) as DeviceType.ResponseDevice<undefined>;
    if (parsedResponse.responseFor === 'GET_DATA') {
      this.deviceData = (parsedResponse.data as unknown) as Alarmclock.Data;
      Device.updateDevice(this.deviceUid, this.deviceData);
    }
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
