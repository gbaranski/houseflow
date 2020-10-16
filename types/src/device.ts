import { Alarmclock, Relay } from '.';

export type AnyDeviceData = Alarmclock.Data | Relay.Data;

export namespace Device {
  export type DeviceType = 'ALARMCLOCK' | 'WATERMIXER' | 'GATE' | 'GARAGE';

  export interface Request {
    correlationData: string;
  }

  export interface FirebaseDevice<
    DeviceData extends
      | Alarmclock.Data
      | Relay.Data
      | AnyDeviceData = AnyDeviceData
  > {
    uid: string;
    secret?: string;
    type: DeviceType;

    status: boolean;
    data: DeviceData;
    ip: string;
  }
}
