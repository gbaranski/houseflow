import WebSocket from 'ws';

export interface Watermixer {
  status: boolean;
  data: WatermixerData;
  ws: WebSocket | undefined;
}

export interface WatermixerData {
  remainingSeconds: number;
  isTimerOn: boolean; // 1 or 0
}

export const watermixerSample: Watermixer = {
  status: false,
  data: {
    remainingSeconds: 0,
    isTimerOn: true,
  },
  ws: undefined,
};
