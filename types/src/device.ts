import { Alarmclock, Watermixer } from '.';
import { DateTime } from './misc';

export type AnyDeviceData = Alarmclock.Data | Watermixer.Data;

export namespace Device {
  export type DeviceType = 'ALARMCLOCK' | 'WATERMIXER' | 'GATE' | 'GARAGE';

  export type RequestType =
    | 'GET_DATA'
    | 'START_MIXING'
    | 'SET_TIME'
    | 'SET_STATE'
    | 'TEST_SIREN'
    | 'REBOOT'
    | 'UNKNOWN';
  export type RequestDevice = ((
    type: 'GET_DATA' | 'START_MIXING' | 'REBOOT',
  ) => any) &
    ((type: 'SET_TIME', data: DateTime) => any) &
    ((type: 'SET_STATE', data: boolean) => any);

  export interface ResponseDevice<
    T extends Alarmclock.Data | Watermixer.Data | undefined
  > {
    ok: boolean;
    responseFor: RequestType;
    data: T;
  }

  export interface FirebaseDevice {
    type: DeviceType;
    secret: string;
    uid: string;
  }
}
