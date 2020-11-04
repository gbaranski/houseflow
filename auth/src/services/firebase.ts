import * as admin from 'firebase-admin';
import { Client, Device } from '@houseflow/types';

export type DocumentReference = admin.firestore.DocumentReference;

admin.initializeApp();

const db = admin.firestore();
const auth = admin.auth();
const deviceCollection = db.collection('devices');
const devicePrivateCollection = db.collection('devices-private');

export let firebaseUsers: Client.FirebaseUser[] = [];

export const validateDevice = async ({
  uid,
  secret,
}: {
  uid: string;
  secret: string;
}) => {
  const deviceData = (await devicePrivateCollection.doc(uid).get()).data() as {
    secret: string;
  };
  if (deviceData.secret !== secret) {
    console.log({ fsS: deviceData.secret, secret });
    throw new Error('secret missmatch');
  }
};

export const decodeToken = (token: string) => auth.verifyIdToken(token);

export async function findDeviceInDatabase(uid: string) {
  const snapshot = await deviceCollection.doc(uid).get();
  if (!snapshot.exists) throw new Error("Device doesn't exist");

  const firebaseDevice = snapshot.data() as Device.FirebaseDevice;
  if (!firebaseDevice.type || !firebaseDevice.uid) {
    throw new Error('Invalid firebase device');
  }
  return firebaseDevice;
}
