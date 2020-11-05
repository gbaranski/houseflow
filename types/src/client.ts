import { ReadWriteExecuteAccess } from './misc';

export namespace Client {
  export type UserRole = 'admin' | 'user';
  export interface DeviceRequest {
    user: {
      token: string;
    };
    device: {
      uid: string;
      gpio: number;
      action: 'trigger' | 'toggle';
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
