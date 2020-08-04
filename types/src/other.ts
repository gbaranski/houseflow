import { AlarmclockData, WatermixerData } from './';

export interface DateTime {
  hour: number;
  minute: number;
  second: number;
}

export enum RequestTypes {
  GET_DATA,
  SET_STATE,
  SET_TIME,
  START_MIXING,
}

export type State = boolean;

export type RequestDevice = ((
  type: RequestTypes.SET_TIME,
  data: DateTime,
) => any) &
  ((type: RequestTypes.SET_STATE, data: boolean) => any);

export interface ResponseDevice {
  ok: boolean;
  responseFor: RequestTypes;
  data?: AlarmclockData | WatermixerData | 'OK';
}

export interface TempHistory {
  unixTime: number;
  temperature: number;
}
export interface RequestHistory {
  user: string;
  requestPath: string;
  unixTime: number;
  ip: string;
  userAgent: string;
  country: string;
}
