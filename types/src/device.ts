import { Alarmclock, Watermixer } from '.';
import { DateTime, State } from './misc';

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

  export type RequestTopic = 'startmix';

  export interface RequestDevice {
    topic: {
      name: RequestTopic;
      uid: string;
    };
    data?: DateTime | State;
  }

  export interface ResponseDevice<
    T extends Alarmclock.Data | Watermixer.Data | undefined
    > {
    responseFor: RequestType;
    data: T;
  }

  export interface FirebaseDevice {
    type: DeviceType;
    secret?: string;
    uid: string;
  }
  export interface ActiveDevice<
    DeviceData extends
    | Alarmclock.Data
    | Watermixer.Data
    | AnyDeviceData = AnyDeviceData
    > extends FirebaseDevice {
    data: DeviceData;
    ip: string;
  }
}
