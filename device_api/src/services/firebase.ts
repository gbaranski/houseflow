import * as admin from 'firebase-admin';
import { Client, Device, Exceptions, RequestHistory } from '@houseflow/types';
import chalk from 'chalk';
import { firestore } from 'firebase-admin';

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

export enum Access {
  READ,
  WRITE,
  EXECUTE,
}
export const checkUserDeviceAccess = ({
  userUid,
  deviceUid,
  access,
}: {
  userUid: string;
  deviceUid: string;
  access: Access;
}): boolean => {
  const firebaseUser = firebaseUsers.find(
    (_firebaseUser) => _firebaseUser.uid === userUid,
  );
  if (!firebaseUser) throw new Error(Exceptions.NO_USER_IN_DB);
  const device = firebaseUser.devices.find(
    (device) => device.uid === deviceUid,
  );
  if (!device) throw new Error(Exceptions.NO_DEVICE_ACCESS);
  if (access === Access.READ && !device.read)
    throw new Error(Exceptions.NO_DEVICE_ACCESS);
  if (access === Access.WRITE && !device.write)
    throw new Error(Exceptions.NO_DEVICE_ACCESS);
  if (access === Access.EXECUTE && !device.execute)
    throw new Error(Exceptions.NO_DEVICE_ACCESS);

  return true;
};

export const addRequestHistory = async ({
  request,
  ipAddress,
  userUid,
}: {
  request: Client.DeviceRequest;
  ipAddress: string;
  userUid: string;
}): Promise<void> => {
  try {
    const targetFirebaseDevice = await devicesCollection
      .doc(request.device.uid)
      .get();
    if (!targetFirebaseDevice.exists)
      throw new Error(Exceptions.NO_DEVICE_IN_DB);
    const destinationDeviceType = (targetFirebaseDevice.data() as Device.FirebaseDevice)
      .type;
    const sourceFirebaseUser = firebaseUsers.find(
      (firebaseUser) => firebaseUser.uid === userUid,
    );
    if (!sourceFirebaseUser) throw new Error(Exceptions.NO_USER_IN_DB);

    const requestHistory: RequestHistory = {
      type: 'request',
      action: `${request.device.action}_${request.device.gpio}`,
      timestamp: Date.now(),
      source: {
        userUid,
        username: sourceFirebaseUser.username,
        geoPoint: new firestore.GeoPoint(
          request.user.geoPoint.latitude,
          request.user.geoPoint.longitude,
        ),
        ipAddress,
      },
      destination: {
        deviceUid: request.device.uid,
        deviceType: destinationDeviceType,
      },
    };
    const res = await devicesCollection
      .doc(request.device.uid)
      .collection('history')
      .add(requestHistory);
    console.log(
      chalk.greenBright(`Successfully added request history ID: ${res.id}`),
    );
  } catch (e) {
    console.log(
      chalk.redBright(
        `Error occured while adding request history ${e.message}`,
      ),
    );
  }
};

export const decodeToken = (
  token: string,
): Promise<admin.auth.DecodedIdToken> => auth.verifyIdToken(token);
