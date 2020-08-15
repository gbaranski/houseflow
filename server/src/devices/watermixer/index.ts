import WebSocket from 'ws';
import Device from '..';
import {
  Watermixer,
  Device as DeviceType,
  AnyDeviceData,
} from '@gbaranski/types';
import { validateDeviceMessage } from '@/helpers';

export class WatermixerDevice extends Device<Watermixer.Data> {
  constructor(
    ws: WebSocket,
    firebaseDevice: DeviceType.FirebaseDevice,
    activeDevice: DeviceType.ActiveDevice<AnyDeviceData>,
  ) {
    super(ws, firebaseDevice, activeDevice);
  }

  handleMessage(message: WebSocket.Data): void {
    validateDeviceMessage(message);
    const parsedResponse = JSON.parse(
      message as string,
    ) as DeviceType.ResponseDevice<undefined>;
    if (parsedResponse.responseFor === 'GET_DATA') {
      this.deviceData = (parsedResponse.data as unknown) as Watermixer.Data;
      Device.updateDevice(this.firebaseDevice.uid, this.deviceData);
    }
  }
}

export default WatermixerDevice;
