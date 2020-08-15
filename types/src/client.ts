import { DateTime, State, Device } from '.';
import { DocumentReference } from '@firebase/firestore-types';
import { AnyDeviceData } from './device';

export namespace Client {
  export type RequestType = 'CONNECTIONS';

  export interface Request {
    requestType: Device.RequestType | RequestType;
    deviceUid?: string;
    deviceType?: string;
    data?: DateTime | State;
  }

  export type ResponseType = 'DATA' | 'CONNECTIONS';

  export interface Response {
    requestType: ResponseType;
    data?: Device.ActiveDevice<AnyDeviceData>[] | Device.FirebaseDevice[];
  }
  export interface FirebaseUser {
    devices: {
      full_access: DocumentReference[];
    };
    permission: number;
    uid: string;
  }
}
