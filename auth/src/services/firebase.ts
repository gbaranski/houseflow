import * as admin from 'firebase-admin';

export type DocumentReference = admin.firestore.DocumentReference;

admin.initializeApp();

const db = admin.firestore();
export const devicePrivateCollection = db.collection('devices-private');

export interface PrivateDeviceData {
  secret: string;
}

export const validateDevice = async ({
  uid,
  secret,
}: {
  uid: string;
  secret: string;
}) => {
  const snapshot = await devicePrivateCollection.doc(uid).get();
  if (!snapshot.exists) throw new Error('device secret doc does not exist');
  const data = snapshot.data() as PrivateDeviceData;
  if (data.secret != secret) {
    console.log({ expected: secret, found_firestore: data.secret });
    throw new Error('secret mismatch');
  }
};
