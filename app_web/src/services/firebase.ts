import firebase, { User } from 'firebase/app';
import 'firebase/firestore';
import 'firebase/analytics';
import 'firebase/functions';
import 'firebase/auth';
import { AnyDeviceData, Client, Device } from '@houseflow/types';

const firebaseConfig = {
  apiKey: 'AIzaSyBmMGXmxzcPLQMFm6BIPXcKg7iqcUJIiks',
  authDomain: 'houseflow-ece2e.firebaseapp.com',
  databaseURL: 'https://houseflow-ece2e.firebaseio.com',
  projectId: 'houseflow-ece2e',
  storageBucket: 'houseflow-ece2e.appspot.com',
  messagingSenderId: '537708599015',
  appId: '1:537708599015:web:62373ac534686c48347442',
  measurementId: 'G-KDBWTHE717',
};
firebase.initializeApp(firebaseConfig);
firebase.analytics();

const database = firebase.firestore();
const functions = firebase.app().functions('europe-west1');
const usersCollection = database.collection('users');
const deviceCollection = database.collection('devices');

export const firebaseAuth: firebase.auth.Auth = firebase.auth();

const googleProvider = new firebase.auth.GoogleAuthProvider();

export async function signInWithCredentials(
  email: string,
  password: string,
): Promise<firebase.auth.UserCredential> {
  // figure out if this has to be there
  await firebaseAuth.setPersistence(firebase.auth.Auth.Persistence.LOCAL);
  return firebaseAuth.signInWithEmailAndPassword(email, password);
}

export const registerWithCredentials = async (email: string, password: string) => {
  await firebaseAuth.setPersistence(firebase.auth.Auth.Persistence.LOCAL);
  return firebaseAuth.createUserWithEmailAndPassword(email, password);
};

export async function signToGoogleWithPopup(): Promise<firebase.auth.UserCredential> {
  await firebaseAuth.setPersistence(firebase.auth.Auth.Persistence.LOCAL);
  return firebaseAuth.signInWithPopup(googleProvider);
}

export const getFirebaseUser = async (
  user: firebase.User,
): Promise<Client.FirebaseUser | undefined> => {
  console.log('Retreiving firebase user');
  return <Client.FirebaseUser | undefined>(await usersCollection.doc(user.uid).get()).data();
};

export const getIdToken = () => {
  return firebaseAuth.currentUser?.getIdToken();
};

export const subscribeAllowedDevices = (
  firebaseUser: Client.FirebaseUser,
  onUpdate: (firebaseDevice: Device.FirebaseDevice) => void,
) => {
  firebaseUser.devices.forEach((firebaseDevice) => {
    deviceCollection.doc(firebaseDevice.uid).onSnapshot((doc) => {
      const newFirebaseDevice = doc.data();
      if (!newFirebaseDevice) throw new Error('Document data empty');
      console.log('Device update: ', doc.data());
      onUpdate(newFirebaseDevice as Device.FirebaseDevice);
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

export async function updateDeviceData(uid: string, deviceData: AnyDeviceData) {
  await deviceCollection.doc(uid).update({ data: deviceData });
}

export async function addNewDevice(firebaseDevice: Device.FirebaseDevice): Promise<void> {
  await deviceCollection.doc(firebaseDevice.uid).update(firebaseDevice);
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
export async function deleteDevice(device: Device.FirebaseDevice) {
  throw new Error('Not implemented');
}

export const sendPasswordResetEmail = (email: string) => {
  return firebaseAuth.sendPasswordResetEmail(email);
};

export const initializeNewUser = functions.httpsCallable('initializeNewUser');
