import { Uid } from './misc';

export namespace Client {
  export type UserRole = 'admin' | 'moderator' | 'user';

  export interface FirebaseUserDevice {
    notification: boolean;
    name: string;
    uid: Uid;
  }

  export interface FirebaseUser {
    devices: Map<Uid, FirebaseUserDevice>[];
    role: UserRole;
    uid: Uid;
  }
}
