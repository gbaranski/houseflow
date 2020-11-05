import { Device } from '.';

export interface DateTime {
  hour: number;
  minute: number;
  second: number;
}

export interface Topic {
  request: string;
  response: string;
}

export enum Exceptions {
  INVALID_DEVICE_REQUEST = 'INVALID_DEVICE_REQUEST',
  NO_USER_IN_DB = 'NO_USER_IN_DB',
  NO_DEVICE_ACCESS = 'NO_DEVICE_ACCESS',
  MQTT_NOT_CONNECTED = 'MQTT_NOT_CONNECTED',
  DEVICE_TIMED_OUT = 'DEVICE_TIMED_OUT',
  INVALID_ARGUMENTS = 'INVALID_ARGUMENTS',
  SUCCESS = 'SUCCESS',
}

export interface RequestHistory {
  deviceType: Device.DeviceType;
  deviceUid: string;
  ipAddress: string;
  action: string;
  timestamp: number;
  userUid: string;
  username: string;
}
