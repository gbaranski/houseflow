import * as admin from 'firebase-admin';
import {
  RequestHistory,
  TempHistory,
  FirebaseUser,
  FirebaseDevice,
  CurrentDevice,
} from '@gbaranski/types';
import { logAdded } from '@/cli';

// eslint-disable-next-line @typescript-eslint/no-var-requires
const serviceAccount = require('@/config/firebaseConfig.json');

export type DocumentReference = admin.firestore.DocumentReference;

admin.initializeApp({
  credential: admin.credential.cert(serviceAccount),
});

const db = admin.firestore();
const requestsCollection = db.collection('requests');
const temperatureCollection = db.collection('temp-history');
const deviceCollection = db.collection('devices');
const usersCollection = db.collection('users');

export async function saveRequestToDb(history: RequestHistory): Promise<void> {
  const res = await requestsCollection.add(history);
  logAdded(`request to Firestore ID: ${res.id}`);
}

export async function addTemperatureToDb(data: TempHistory): Promise<void> {
  const res = await temperatureCollection.add(data);
  logAdded(`temperature to Firestore ID: ${res.id}`);
}

export async function validateDevice(
  deviceType: string,
  uid: string,
  secret: string,
): Promise<CurrentDevice> {
  const snapshot = await deviceCollection.doc(uid).get();
  if (!snapshot.exists) {
    throw new Error('Does not exist!');
  }

  const snapshotData = snapshot.data() as FirebaseDevice;
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
): Promise<FirebaseUser> {
  if (!uid) throw new Error('User UID is not defined');
  const usersDoc = await usersCollection.doc(uid).get();
  if (!usersDoc.exists) throw new Error('User does not exist in database');
  const usersData = usersDoc.data() as FirebaseUser;
  const firebaseUser: FirebaseUser = {
    ...usersData,
  };
  return firebaseUser;
}
