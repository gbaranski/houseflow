import * as admin from 'firebase-admin';
import { Client, Exceptions } from '@houseflow/types';

export type DocumentReference = admin.firestore.DocumentReference;

admin.initializeApp();

const db = admin.firestore();
const auth = admin.auth();
const usersCollection = db.collection('users');

export let firebaseUsers: Client.FirebaseUser[] = [];

usersCollection.onSnapshot((snapshot) =>
  snapshot.docs.forEach((doc) => {
    const firebaseUser = doc.data() as Client.FirebaseUser;
    firebaseUsers = firebaseUsers
      .filter((user) => user.uid !== firebaseUser.uid)
      .concat(firebaseUser);
  }),
);

export const checkUserDeviceAccess = ({
  userUid,
  deviceUid,
}: {
  userUid: string;
  deviceUid: string;
}): boolean => {
  const firebaseUser = firebaseUsers.find(
    (_firebaseUser) => _firebaseUser.uid === userUid,
  );
  if (!firebaseUser) throw new Error(Exceptions.NO_USER_IN_DB);
  const device = firebaseUser.devices.find(
    (device) => device.uid === deviceUid,
  );
  if (!device) throw new Error(Exceptions.NO_DEVICE_ACCESS);
  return true;
};

export const decodeToken = (
  token: string,
): Promise<admin.auth.DecodedIdToken> => auth.verifyIdToken(token);
