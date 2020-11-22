import { GeoPoint, ReadWriteExecuteAccess } from '.';

export namespace Client {
  export type UserRole = 'admin' | 'user';
  export enum DeviceActionName {
    OpenGate = 'action_open_gate',
    OpenGarage = 'action_open_garage',
    SwitchLights = 'action_switch_lights',
    MixWater = 'action_mix_water',
  }

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
