export namespace Client {
  export type UserRole = 'admin' | 'moderator' | 'user';

  export interface FirebaseUserDevice {
    notification: boolean;
    uid: boolean;
  }

  export interface FirebaseUser {
    devices: FirebaseUserDevice[];
    role: UserRole;
    uid: string;
  }
}
