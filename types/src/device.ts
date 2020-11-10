import { GeoPoint } from '.';

export namespace Device {
  export type DeviceType =
    | 'ALARMCLOCK'
    | 'WATERMIXER'
    | 'GATE'
    | 'GARAGE'
    | 'LIGHT';

  export interface Request {
    correlationData: string;
    data?: unknown | undefined;
  }

  export interface Response {
    correlationData: string;
    data?: unknown | undefined;
  }

  export interface FirebaseDevice<T = undefined> {
    uid: string;
    secret?: string;
    type: DeviceType;
    geoPoint: GeoPoint;

    status: boolean;
    data: T;
    ip: string;
  }
}
