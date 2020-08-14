import * as admin from 'firebase-admin';
import { Device, Client } from '@gbaranski/types';

// eslint-disable-next-line @typescript-eslint/no-var-requires
const serviceAccount = require('@/config/firebaseConfig.json');

export type DocumentReference = admin.firestore.DocumentReference;

admin.initializeApp({
  credential: admin.credential.cert(serviceAccount),
});

const db = admin.firestore();
const devicePrivateCollection = db.collection('devices-private');
const usersCollection = db.collection('users');

export async function validateDevice(
  deviceType: string,
  uid: string,
  secret: string,
): Promise<Device.FirebaseDevice> {
  const snapshot = await devicePrivateCollection.doc(uid).get();
  if (!snapshot.exists) {
    throw new Error('Does not exist!');
  }

  const snapshotData = snapshot.data() as Device.FirebaseDevice;
  if (snapshotData.secret !== secret) {
    console.log({
      currentSecret: snapshotData.secret,
      desiredSecret: secret,
    });
    throw new Error("Device doesn't match");
  }
  return {
    ...snapshotData,
    uid: snapshot.id,
  };
}

export async function decodeClientToken(
  token: string,
): Promise<admin.auth.DecodedIdToken> {
  try {
    const decodedClient = await admin.auth().verifyIdToken(token);
    return decodedClient;
  } catch (e) {
    throw e;
  }
}

export function sendMessage(username: string, requestTypeString: string): void {
  const message = {
    name: 'Alert',
    data: {
      title: 'Home alert!',
      body: `${username} requested ${requestTypeString}!`,
    },
    notification: {
      title: 'Home alert!',
      body: `${username} requested ${requestTypeString}`,
    },
    topic: 'admin',
  };
  admin
    .messaging()
    .send(message)
    .catch((error): void => {
      console.log('Error sending message:', error);
    });
}

export async function convertToFirebaseUser(
  uid: string,
): Promise<Client.FirebaseUser> {
  if (!uid) throw new Error('User UID is not defined');
  const usersDoc = await usersCollection.doc(uid).get();
  if (!usersDoc.exists) throw new Error('User does not exist in database');
  const usersData = usersDoc.data() as Client.FirebaseUser;
  const firebaseUser: Client.FirebaseUser = {
    ...usersData,
  };
  return firebaseUser;
}
