import * as admin from 'firebase-admin';
import { Client, Device, Exceptions, RequestHistory } from '@houseflow/types';

export type DocumentReference = admin.firestore.DocumentReference;

admin.initializeApp();

const db = admin.firestore();
const auth = admin.auth();
const usersCollection = db.collection('users');
const devicesCollection = db.collection('devices');

export let firebaseUsers: Client.FirebaseUser[] = [];

export const usersCollectionListener = usersCollection.onSnapshot((snapshot) =>
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

export const addRequestHistory = async ({
  userUid,
  deviceUid,
  action,
  ipAddress,
}: {
  userUid: string;
  deviceUid: string;
  action: string;
  ipAddress: string;
}): Promise<void> => {
  const targetFirebaseDevice = await devicesCollection.doc(deviceUid).get();
  if (!targetFirebaseDevice.exists) throw new Error(Exceptions.NO_DEVICE_IN_DB);
  const targetDeviceType = (targetFirebaseDevice.data() as Device.FirebaseDevice)
    .type;
  const sourceFirebaseUser = firebaseUsers.find(
    (firebaseUser) => firebaseUser.uid === userUid,
  );
  if (!sourceFirebaseUser) throw new Error(Exceptions.NO_USER_IN_DB);

  const requestHistory: RequestHistory = {
    deviceType: targetDeviceType,
    deviceUid,
    ipAddress,
    action,
    timestamp: Date.now(),
    userUid,
    username: sourceFirebaseUser.username,
  };
  await devicesCollection
    .doc(deviceUid)
    .collection('history')
    .add(requestHistory);
};

export const decodeToken = (
  token: string,
): Promise<admin.auth.DecodedIdToken> => auth.verifyIdToken(token);
