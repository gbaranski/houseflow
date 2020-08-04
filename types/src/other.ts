import { AlarmclockData, WatermixerData, Alarmclock } from './';
import { Watermixer } from './watermixer';

export interface Devices {
  alarmclock: Alarmclock;
  watermixer: Watermixer;
}

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
  REBOOT,
  UNKNOWN,
}

export enum DeviceType {
  ALARMCLOCK,
  WATERMIXER,
  GATE,
  GARAGE,
}

export type DevicesTypes = keyof typeof DeviceType;

export type State = boolean;

export type AnyDeviceData = AlarmclockData | WatermixerData;

export type RequestDevice = ((
  type: RequestTypes.SET_TIME,
  data: DateTime,
) => any) &
  ((type: RequestTypes.SET_STATE, data: boolean) => any);

export interface ResponseDevice<
  T extends AlarmclockData | WatermixerData | undefined
> {
  ok: boolean;
  responseFor: RequestTypes;
  data: T;
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
