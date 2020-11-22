import { GeoPoint, ReadWriteExecuteAccess } from '.';

export namespace Client {
  export type UserRole = 'admin' | 'user';
  export type DeviceActionName =
    | 'action_open_gate'
    | 'action_mix_water'
    | 'action_open_garage'
    | 'action_turn_on_lights';

  export interface DeviceRequestUser {
    token: string;
    geoPoint: GeoPoint;
  }

  export interface DeviceRequestByActionName {
    user: DeviceRequestUser;
    device: {
      action: DeviceActionName;
      data?: string;
    };
  }

  export interface DeviceRequestByUID {
    user: DeviceRequestUser;
    device: {
      uid: string;
      action: number;
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
