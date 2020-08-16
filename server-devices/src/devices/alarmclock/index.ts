import WebSocket from 'ws';
import Device from '..';
import {
  Alarmclock,
  Device as DeviceType,
  AnyDeviceData,
} from '@gbaranski/types';
import { validateDeviceMessage } from '@/services/misc';

export default class AlarmclockDevice extends Device<Alarmclock.Data> {
  constructor(
    ws: WebSocket,
    firebaseDevice: DeviceType.FirebaseDevice,
    activeDevice: DeviceType.ActiveDevice,
  ) {
    super(ws, firebaseDevice, activeDevice);
  }
  handleMessage(message: WebSocket.Data): void {
    validateDeviceMessage(message);
    const parsedResponse = JSON.parse(
      message as string,
    ) as DeviceType.ResponseDevice<undefined>;
    if (parsedResponse.responseFor === 'GET_DATA') {
      this.deviceData = (parsedResponse.data as unknown) as Alarmclock.Data;
      Device.updateDevice(this.firebaseDevice.uid, this.deviceData);
    }
  }
}
