import { DocumentReference } from '@firebase/firestore-types';
import { DeviceType } from './other';

export interface FirebaseUser {
  devices: {
    full_access: DocumentReference[];
  };
  permission: number;
}

export interface FirebaseDevice {
  secret: string;
  uid: string;
  type: DeviceType;
}
