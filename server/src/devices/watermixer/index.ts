import WebSocket from 'ws';
import Device from '..';
import { Watermixer, Device as DeviceType } from '@gbaranski/types';
import { validateDeviceMessage } from '@/helpers';

export class WatermixerDevice extends Device<Watermixer.Data> {
  constructor(ws: WebSocket, device: DeviceType.FirebaseDevice) {
    super(ws, Watermixer.SAMPLE, 'WATERMIXER', device.uid, device.secret);
  }

  handleMessage(message: WebSocket.Data): void {
    validateDeviceMessage(message);
    const parsedResponse = JSON.parse(
      message as string,
    ) as DeviceType.ResponseDevice<undefined>;
    if (parsedResponse.responseFor === 'GET_DATA') {
      this.deviceData = (parsedResponse.data as unknown) as Watermixer.Data;
      Device.updateDevice(this.deviceUid, this.deviceData);
    }
  }
}

export default WatermixerDevice;
