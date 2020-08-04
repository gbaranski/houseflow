import WebSocket from 'ws';
import { DateTime } from './';

export interface AlarmclockData {
  alarmTime: DateTime;
  alarmState: boolean;
  sensor: {
    temperature: number;
    humidity: number;
    heatIndex: number;
  };
}

export const alarmclockSample: AlarmclockData = {
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

export interface Alarmclock {
  status: boolean;
  data: AlarmclockData;
  ws: WebSocket | undefined;
}
