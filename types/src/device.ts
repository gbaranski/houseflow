import { Alarmclock, Watermixer } from '.';
import { DateTime, State, Uid } from './misc';

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
      uid: Uid;
    };
    data?: DateTime | State;
  }

  export interface ResponseDevice<
    T extends Alarmclock.Data | Watermixer.Data | undefined
  > {
    responseFor: RequestType;
    data: T;
  }

  export interface FirebaseDevice<
    DeviceData extends
      | Alarmclock.Data
      | Watermixer.Data
      | AnyDeviceData = AnyDeviceData
  > {
    uid: Uid;
    secret?: string;
    type: DeviceType;

    status: boolean;
    data: DeviceData;
    ip: string;
  }
}
