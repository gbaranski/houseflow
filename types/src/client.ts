export namespace Client {
  // TODO: Remove moderator
  export type UserRole = 'admin' | 'moderator' | 'user';

  export interface FirebaseUserDevice {
    uid: string;
  }

  export interface FirebaseUser {
    devices: FirebaseUserDevice[];
    role: UserRole;
    uid: string;
    username: string;
  }
}
