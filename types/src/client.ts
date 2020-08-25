import { DateTime, State, Device } from '.';
import { DocumentReference } from '@firebase/firestore-types';
import { CurrentConnections } from './misc';

export namespace Client {
  export type RequestType = 'CONNECTIONS';

  export interface Request {
    requestType: Device.RequestType | RequestType;
    deviceUid?: string;
    deviceType?: string;
    data?: DateTime | { state: State };
  }

  export type ResponseType = 'DATA' | 'CONNECTIONS';
  export type UserRole = 'admin' | 'moderator' | 'user';

  export interface Response {
    requestType: ResponseType;
    data?: Device.ActiveDevice[] | Device.FirebaseDevice[] | CurrentConnections;
  }
  export interface FirebaseUser {
    devices: DocumentReference[];
    role: UserRole;
    uid: string;
  }
  export interface ActiveUser extends FirebaseUser {
    ip: string;
  }
}
