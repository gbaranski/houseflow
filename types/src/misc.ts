import { Device } from '.';

export interface Topic {
  request: string;
  response: string;
}

export enum Exceptions {
  INVALID_DEVICE_REQUEST = 'INVALID_DEVICE_REQUEST',
  NO_USER_IN_DB = 'NO_USER_IN_DB',
  NO_DEVICE_IN_DB = 'NO_DEVICE_IN_DB',
  NO_DEVICE_ACCESS = 'NO_DEVICE_ACCESS',
  MQTT_NOT_CONNECTED = 'MQTT_NOT_CONNECTED',
  DEVICE_TIMED_OUT = 'DEVICE_TIMED_OUT',
  INVALID_ARGUMENTS = 'INVALID_ARGUMENTS',
  SUCCESS = 'SUCCESS',
}

export interface GeoPoint {
  latitude: number;
  longitude: number;
}

export interface RequestHistory {
  type: 'request';
  timestamp: number;
  action: string;
  source: {
    userUid: string;
    username: string;

    geoPoint: GeoPoint;
    ipAddress: string;
  };
  destination: {
    deviceUid: string;
    deviceType: Device.DeviceType;
  };
}

export interface ReadWriteExecuteAccess {
  read: boolean;
  write: boolean;
  execute: boolean;
}
