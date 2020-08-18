import WebSocket from 'ws';
import Device from '../';
import {
  Watermixer,
  Device as DeviceType,
  AnyDeviceData,
} from '@gbaranski/types';
import { validateDeviceMessage } from '@/services/misc';

export class WatermixerDevice extends Device<Watermixer.Data> {
  constructor(
    ws: WebSocket,
    firebaseDevice: DeviceType.FirebaseDevice,
    activeDevice: DeviceType.ActiveDevice,
  ) {
    super(ws, firebaseDevice, activeDevice);
  }

  public handleMessage(message: WebSocket.Data): void {
    validateDeviceMessage(message);
    const parsedResponse = JSON.parse(
      message as string,
    ) as DeviceType.ResponseDevice<undefined>;
    if (parsedResponse.responseFor === 'GET_DATA') {
      const activeDevice = {
        ...this.activeDevice,
        data: (parsedResponse.data as unknown) as AnyDeviceData,
      };
      this.updateDevice(activeDevice);
      this.activeDevice = activeDevice;
    }
  }
}

export default WatermixerDevice;
