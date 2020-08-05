import WebSocket from 'ws';
import { RequestTypes } from '.';

export interface Watermixer {
  status: boolean;
  data: WatermixerData;
  ws: WebSocket | undefined;
}

export type RequestWatermixer = ((type: RequestTypes.GET_DATA) => any) &
  ((type: RequestTypes.START_MIXING) => any) &
  ((type: RequestTypes.REBOOT) => any);

export interface WatermixerData {
  remainingSeconds: number;
  isTimerOn: boolean; // 1 or 0
}

export const watermixerSample: WatermixerData = {
  remainingSeconds: 0,
  isTimerOn: true,
};
