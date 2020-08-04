import WebSocket from 'ws';
import { v4 as uuidv4 } from 'uuid';
import { watermixerSample, WatermixerData } from '@gbaranski/types';
import Device from '..';

export class WatermixerDevice extends Device<WatermixerData> {
  dataInterval = setInterval(() => {
    this.interval();
  }, 500);

  constructor(ws: WebSocket) {
    super(ws, watermixerSample, 'WATERMIXER', uuidv4());
  }
}

export default WatermixerDevice;
