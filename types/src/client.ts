export namespace Client {
  export type UserRole = 'admin' | 'moderator' | 'user';

  export interface FirebaseUser {
    devices: { notification: boolean; uid: boolean }[];
    role: UserRole;
    uid: string;
  }
}
