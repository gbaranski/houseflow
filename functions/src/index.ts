import { Client } from '@houseflow/types';
import * as functions from 'firebase-functions';
import * as admin from 'firebase-admin';

admin.initializeApp();
const firestore = admin.firestore();
const usersCollection = firestore.collection('users');

interface InitUserData {
  username: string;
}

exports.initializeNewUser = functions
  .region('europe-west1')
  .https.onCall(async (data, context) => {
    const uid = context.auth?.uid;
    if (!uid)
      throw new functions.https.HttpsError(
        'unauthenticated',
        'User is not logged in',
      );
    const request: InitUserData = data;
    if (!request.username)
      throw new functions.https.HttpsError(
        'invalid-argument',
        'Missing username parameter',
      );

    const snapshot = await usersCollection.doc(uid).get();
    if (snapshot.exists)
      throw new functions.https.HttpsError(
        'aborted',
        'User already exists in firestore',
      );

    const firebaseUser: Client.FirebaseUser = {
      devices: [],
      role: 'user',
      uid,
      username: request.username,
    };
    await usersCollection.doc(uid).set(firebaseUser);
  });
