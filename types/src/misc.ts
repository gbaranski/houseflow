import { Device, AnyDeviceData } from './device';
import { Client } from './client';

export interface DateTime {
  hour: number;
  minute: number;
  second: number;
}

export type State = boolean;

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

export interface CurrentConnections {
  devices: Device.ActiveDevice<AnyDeviceData>[];
  clients: Client.ActiveUser[];
}
