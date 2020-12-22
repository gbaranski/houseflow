import * as admin from 'firebase-admin';

export type DocumentReference = admin.firestore.DocumentReference;

admin.initializeApp();

const db = admin.firestore();
const auth = admin.auth();
export const usersCollection = db.collection('users');

export const getFirebaseUserByGoogleClientID = (
  uid: string,
): Promise<admin.auth.GetUsersResult> => {
  return auth.getUsers([
    {
      providerId: 'google',
      providerUid: uid,
    },
  ]);
};
