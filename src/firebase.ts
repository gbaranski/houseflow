import * as admin from 'firebase-admin';
import { RequestHistory } from '@gbaranski/types';

// eslint-disable-next-line @typescript-eslint/no-var-requires
const serviceAccount = require('./firebaseConfig.json');

admin.initializeApp({
  credential: admin.credential.cert(serviceAccount),
  databaseURL: 'https://controlhome-7bbcc.firebaseio.com/',
});

const db = admin.database();
const ref = db.ref('/');
const requestsRef = ref.child('/requests');

export function saveRequestToDb(history: RequestHistory): void {
  requestsRef.push(history);
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
    // eslint-disable-next-line @typescript-eslint/explicit-function-return-type
    .then((response): void => {
      console.log('Successfully sent message:', response);
    })
    .catch((error): void => {
      console.log('Error sending message:', error);
    });
}
