import * as admin from 'firebase-admin';
import { RequestHistory, TempHistory } from '@gbaranski/types';
import { logAdded } from './cli';

// eslint-disable-next-line @typescript-eslint/no-var-requires
const serviceAccount = require('./firebaseConfig.json');

admin.initializeApp({
  credential: admin.credential.cert(serviceAccount),
});

const db = admin.firestore();
const requestsCollection = db.collection('requests');
const temperatureCollection = db.collection('temp-history');

export async function saveRequestToDb(history: RequestHistory): Promise<void> {
  const res = await requestsCollection.add(history);
  logAdded(`request to Firestore ID: ${res.id}`);
}

export async function addTemperatureToDb(data: TempHistory): Promise<void> {
  const res = await temperatureCollection.add(data);
  logAdded(`temperature to Firestore ID: ${res.id}`);
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
