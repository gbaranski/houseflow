import WebSocket from 'ws';
import { DateTime } from './';

export namespace Alarmclock {
  export interface Active {
    status: boolean;
    data: Data;
    ws: WebSocket | undefined;
  }

  export interface Data {
    alarmTime: DateTime;
    alarmState: boolean;
    sensor: {
      temperature: number;
      humidity: number;
      heatIndex: number;
    };
  }

  export const SAMPLE: Data = {
    alarmTime: {
      hour: 0,
      minute: 0,
      second: 0,
    },
    alarmState: false,
    sensor: {
      temperature: 0,
      humidity: 0,
      heatIndex: 0,
    },
  };
}
