import { DateTime, State, Device } from '.';
import { DocumentReference } from '@firebase/firestore-types';
import { AnyDeviceData } from './device';
import { CurrentConnections } from './misc';

export namespace Client {
  export type RequestType = 'CONNECTIONS';

  export interface Request {
    requestType: Device.RequestType | RequestType;
    deviceUid?: string;
    deviceType?: string;
    data?: DateTime | State;
  }

  export type ResponseType = 'DATA' | 'CONNECTIONS';
  export type UserRole = 'admin' | 'moderator' | 'user';

  export interface Response {
    requestType: ResponseType;
    data?: Device.ActiveDevice[] | Device.FirebaseDevice[] | CurrentConnections;
  }
  export interface FirebaseUser {
    devices: {
      full_access: DocumentReference[];
    };
    role: UserRole;
    uid: string;
  }
  export interface ActiveUser extends FirebaseUser {
    ip: string;
  }
}
