import WebSocket from 'ws';
import Device from '..';
import { Alarmclock, Device as DeviceType } from '@gbaranski/types';
import { validateDeviceMessage } from '@/helpers';

export default class AlarmclockDevice extends Device<Alarmclock.Data> {
  constructor(ws: WebSocket, device: DeviceType.FirebaseDevice) {
    super(ws, Alarmclock.SAMPLE, 'ALARMCLOCK', device.uid);
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
}
