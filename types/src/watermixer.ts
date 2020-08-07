import WebSocket from 'ws';

export namespace Watermixer {
  export interface Active {
    status: boolean;
    data: Data;
    ws: WebSocket | undefined;
  }

  export interface Data {
    remainingSeconds: number;
    isTimerOn: boolean; // 1 or 0
  }

  export const SAMPLE: Data = {
    remainingSeconds: 0,
    isTimerOn: true,
  };
}
