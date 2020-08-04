import {
  IAlarmclockData,
  IWatermixerData,
  IAlarmclock,
  TRequestAlarmclock,
  IWatermixer,
  TRequestWatermixer,
  AlarmclockRequestTypes,
} from './';

export interface IDevices {
  alarmclock: IAlarmclock;
  watermixer: IWatermixer;
}

export interface IDateTime {
  hour: number;
  minute: number;
  second: number;
}

export enum DeviceType {
  ALARMCLOCK = 'ALARMCLOCK',
  WATERMIXER = 'WATERMIXER',
  GATE = 'GATE',
  GARAGE = 'GARAGE',
}

export type TDevicesTypes = keyof typeof DeviceType;

export type TState = boolean;

export type TAnyDeviceData = IAlarmclockData | IWatermixerData;
export type TAnyDevice = IWatermixer | IAlarmclock;

export type TRequestType<
  Device extends IAlarmclock | IWatermixer
> = Device extends IAlarmclock
  ? AlarmclockRequestTypes
  : Device extends IWatermixer
  ? AlarmclockRequestTypes
  : undefined;

export type TRequestDevice<
  Device extends IAlarmclock | IWatermixer
> = Device extends IAlarmclock
  ? TRequestAlarmclock
  : Device extends IWatermixer
  ? TRequestWatermixer
  : undefined;

export interface ITempHistory {
  unixTime: number;
  temperature: number;
}
export interface IRequestHistory {
  user: string;
  requestPath: string;
  unixTime: number;
  ip: string;
  userAgent: string;
  country: string;
}
