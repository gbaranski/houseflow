import WebSocket from 'ws';

export namespace Watermixer {
  export interface Active {
    status: boolean;
    data: Data;
    ws: WebSocket | undefined;
  }

  export interface Data {
    finishMixTimestamp: number;
    isTimerOn: boolean;
  }

  export const SAMPLE: Data = {
    finishMixTimestamp: 0,
    isTimerOn: true,
  };
}
