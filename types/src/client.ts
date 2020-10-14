import { Uid } from './misc';

export namespace Client {
  // TODO: Remove moderator
  export type UserRole = 'admin' | 'moderator' | 'user';

  export interface FirebaseUserDevice {
    name: string;
    uid: Uid;
  }

  export interface FirebaseUser {
    devices: FirebaseUserDevice[];
    role: UserRole;
    uid: Uid;
    username: string;
  }
}
