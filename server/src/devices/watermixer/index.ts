import WebSocket from 'ws';
import { v4 as uuidv4 } from 'uuid';
import {
  watermixerSample,
  WatermixerData,
  ResponseDevice,
  RequestTypes,
  CurrentDevice,
  DeviceType,
} from '@gbaranski/types';
import Device from '..';
import { validateDeviceMessage } from '@/helpers';

export class WatermixerDevice extends Device<WatermixerData> {
  constructor(ws: WebSocket, device: CurrentDevice) {
    super(ws, watermixerSample, DeviceType.WATERMIXER, device.uid);
  }

  handleMessage(message: WebSocket.Data): void {
    validateDeviceMessage(message);
    const parsedResponse = JSON.parse(message as string) as ResponseDevice<
      undefined
    >;
    if (parsedResponse.responseFor === RequestTypes.GET_DATA) {
      this.deviceData = (parsedResponse.data as unknown) as WatermixerData;
    }
  }
}

export default WatermixerDevice;
