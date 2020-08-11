import { DateTime, State, Device } from '.';
import { DocumentReference } from '@firebase/firestore-types';
import { AnyDeviceData } from './device';

export namespace Client {
  export interface Request {
    requestType: Device.RequestType;
    deviceUid?: string;
    deviceType?: string;
    data?: DateTime | State;
  }

  export type ResponseType = 'DATA' | 'DEVICES' | 'DEVICES_STATUS';

  export interface Response {
    requestType: ResponseType;
    data?: Device.ActiveDevice<AnyDeviceData>[] | Device.FirebaseDevice[];
  }
  export interface FirebaseUser {
    devices: {
      full_access: DocumentReference[];
    };
    permission: number;
  }
}
