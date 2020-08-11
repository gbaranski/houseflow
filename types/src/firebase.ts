import { DocumentReference } from '@firebase/firestore-types';
import { DeviceType } from './other';

export interface FirebaseUser {
  devices: {
    fullAccess: DocumentReference[];
  };
  permission: number;
}

export interface FirebaseDevice {
  secret: string;
  type: DeviceType;
}
