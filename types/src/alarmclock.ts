import WebSocket from 'ws';
import { IDateTime } from './';

export interface IAlarmclockData {
  alarmTime: IDateTime;
  alarmState: boolean;
  sensor: {
    temperature: number;
    humidity: number;
    heatIndex: number;
  };
}

export const alarmclockSample: IAlarmclockData = {
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

export enum AlarmclockRequestTypes {
  GET_DATA = 'GET_DATA',
  START_MIXING = 'START_MIXING',
  SET_TIME = 'SET_TIME',
  SET_STATE = 'SET_STATE',
  TEST_SIREN = 'TEST_SIREN',
  REBOOT = 'REBOOT',
  UNKNOWN = 'UNKNOWN',
}
export type TRequestAlarmclock = ((
  type: AlarmclockRequestTypes.GET_DATA,
) => any) &
  ((type: AlarmclockRequestTypes.TEST_SIREN) => any) &
  ((type: AlarmclockRequestTypes.SET_TIME, data: IDateTime) => any) &
  ((type: AlarmclockRequestTypes.SET_STATE, data: boolean) => any) &
  ((type: AlarmclockRequestTypes.REBOOT) => any);

export interface IAlarmclock {
  data: IAlarmclockData;
}
