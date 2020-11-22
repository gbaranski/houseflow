import { GeoPoint, ReadWriteExecuteAccess } from '.';
import { Device } from './device';

export namespace Client {
  export type UserRole = 'admin' | 'user';

  export interface DeviceRequest {
    user: {
      token: string;
      geoPoint: GeoPoint;
    };
    device: {
      action: Device.Action;
      uid?: string;
      data?: string;
    };
  }

  export interface FirebaseUserDevice extends ReadWriteExecuteAccess {
    uid: string;
  }

  export interface FirebaseUser {
    devices: FirebaseUserDevice[];
    role: UserRole;
    uid: string;
    username: string;
  }
}
