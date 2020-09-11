import * as admin from 'firebase-admin';
import { Device } from '@gbaranski/types';

// eslint-disable-next-line @typescript-eslint/no-var-requires
const serviceAccount = require('@/firebaseConfig.json');

export type DocumentReference = admin.firestore.DocumentReference;

admin.initializeApp({
  credential: admin.credential.cert(serviceAccount),
});

const db = admin.firestore();
const deviceCollection = db.collection('devices');
const devicePrivateCollection = db.collection('devices-private');

export interface DeviceCredentials {
  uid: string;
  secret: string;
}

export async function validateDevice({ uid, secret }: DeviceCredentials): Promise<{ secret: string }> {
  const snapshot = await devicePrivateCollection.doc(uid).get();
  if (!snapshot.exists) throw new Error('Does not exist!');

  const snapshotData = snapshot.data() as { secret: string };
  if (snapshotData.secret !== secret) {
    console.log({
      currentSecret: snapshotData.secret,
      desiredSecret: secret,
    });
    throw new Error("Device doesn't match");
  }
  return snapshotData;
}


export async function findDeviceInDatabase(uid: string) {
  const snapshot = await deviceCollection.doc(uid).get();
  if (!snapshot.exists) throw new Error("Device doesn't exist");

  const firebaseDevice = snapshot.data() as Device.FirebaseDevice;
  if (!firebaseDevice.type || !firebaseDevice.uid) {
    throw new Error("Invalid firebase device");
  }
  return firebaseDevice;
}
