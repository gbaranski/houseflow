import firebase, { User } from 'firebase/app';
import 'firebase/firebase-database';
import 'firebase/analytics';
import 'firebase/auth';
import { Client, Device, Alarmclock, Watermixer, AnyDeviceData } from '@gbaranski/types';

const firebaseConfig = {
  apiKey: 'AIzaSyAC2m1CB6x8J4YXnmmkdY6LL9cW3xXjNwM',
  authDomain: 'controlhome.firebaseapp.com',
  databaseURL: 'https://controlhome.firebaseio.com',
  projectId: 'controlhome',
  storageBucket: 'controlhome.appspot.com',
  messagingSenderId: '917528821196',
  appId: '1:917528821196:web:49e832b53c90b6aab57169',
  measurementId: 'G-GET25DVWHE',
};

const app = firebase.initializeApp(firebaseConfig);
firebase.analytics();

const database = firebase.database();
const usersRef = database.ref('users');
const deviceRef = database.ref('devices');

export const firebaseAuth: firebase.auth.Auth = app.auth();

const googleProvider = new firebase.auth.GoogleAuthProvider();

export async function signInWithCredentials(
  email: string,
  password: string,
): Promise<firebase.auth.UserCredential> {
  // figure out if this has to be there
  await firebaseAuth.setPersistence(firebase.auth.Auth.Persistence.LOCAL);
  return firebaseAuth.signInWithEmailAndPassword(email, password);
}

export async function signToGoogleWithPopup(): Promise<firebase.auth.UserCredential> {
  await firebaseAuth.setPersistence(firebase.auth.Auth.Persistence.LOCAL);
  return firebaseAuth.signInWithPopup(googleProvider);
}

export const getUser = async (user: firebase.User): Promise<Client.FirebaseUser> => {
  if (!user) throw new Error('User is not defined');
  console.log('Retreiving firebase user');

  return new Promise<Client.FirebaseUser>((resolve) => {
    usersRef
      .child(user.uid)
      .once('value')
      .then((snapshot) => {
        const firebaseUser = snapshot.val();
        console.log(firebaseUser);
        resolve(firebaseUser);
      });
  });
};

export const getDevice = (uid: string): Promise<Device.FirebaseDevice> => {
  return new Promise<Device.FirebaseDevice>((resolve) => {
    deviceRef
      .child(uid)
      .once('value')
      .then((snapshot) => resolve(snapshot.val()));
  });
};

export async function getIdToken(): Promise<string> {
  if (!firebaseAuth.currentUser)
    throw new Error('Cannot retreive ID token cause currentUser not defined');
  return firebaseAuth.currentUser.getIdToken(true);
}

export function getSampleData(deviceType: Device.DeviceType): AnyDeviceData {
  switch (deviceType) {
    case 'ALARMCLOCK':
      return Alarmclock.SAMPLE;
    case 'WATERMIXER':
      return Watermixer.SAMPLE;
    default:
      return Watermixer.SAMPLE;
  }
}

export async function getAllowedDevices(
  firebaseUser: Client.FirebaseUser,
): Promise<Device.FirebaseDevice[]> {
  const currentDevices = firebaseUser.devices.map(async (userDevice) => getDevice(userDevice.uid));

  return await Promise.all(currentDevices);
}

export const subscribeAllowedDevices = (
  firebaseUser: Client.FirebaseUser,
  onUpdate: (firebaseDevice: Device.FirebaseDevice) => void,
) => {
  firebaseUser.devices.forEach((device) => {
    deviceRef.child(device.uid).on('value', (snapshot) => {
      console.log('Device update: ', snapshot.val());
      onUpdate(snapshot.val());
    });
  });
};

let userLoaded: boolean = false;

export function getCurrentUser(): Promise<User | undefined> {
  return new Promise<User | undefined>((resolve, reject) => {
    if (userLoaded) {
      resolve(firebaseAuth.currentUser || undefined);
    }
    const unsubscribe = firebaseAuth.onAuthStateChanged((user) => {
      userLoaded = true;
      unsubscribe();
      resolve(user ?? undefined);
    }, reject);
  });
}

export async function addNewDevice(firebaseDevice: Device.FirebaseDevice): Promise<void> {
  throw new Error('Not implemented');
}

export async function getAllDevices(): Promise<Device.FirebaseDevice[]> {
  throw new Error('Not implemented');
}

export async function deleteDevice(device: Device.FirebaseDevice) {
  throw new Error('Not implemented');
}
