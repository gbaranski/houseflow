import { DocumentReference } from '@firebase/firestore-types';
import { Device } from './device';

export interface FirebaseUser {
  devices: {
    full_access: DocumentReference[];
  };
  permission: number;
}

export interface FirebaseDevice {
  secret: string;
  type: Device.DeviceType;
}
