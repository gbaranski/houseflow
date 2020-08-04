import WebSocket from 'ws';
import { v4 as uuidv4 } from 'uuid';
import {
  watermixerSample,
  WatermixerData,
  ResponseDevice,
  RequestTypes,
} from '@gbaranski/types';
import Device from '..';

export class WatermixerDevice extends Device<WatermixerData> {
  dataInterval = setInterval(() => {
    this.interval();
  }, 500);

  constructor(ws: WebSocket) {
    super(ws, watermixerSample, 'WATERMIXER', uuidv4());
  }

  handleMessage(message: WebSocket.Data): void {
    // validateSocketMessage(message);
    const parsedResponse = JSON.parse(message as string) as ResponseDevice<
      undefined
    >;
    if (parsedResponse.responseFor === RequestTypes.GET_DATA) {
      this.deviceData = (parsedResponse.data as unknown) as WatermixerData;
    }
  }
}

export default WatermixerDevice;
