import firebase, { User } from 'firebase/app';
import 'firebase/firestore';
import 'firebase/analytics';
import 'firebase/auth';
import { Client, Device } from '@gbaranski/types';

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

const database = firebase.firestore();
const usersCollection = database.collection('users');
const deviceCollection = database.collection('devices');

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

export const getUser = async (user: firebase.User): Promise<Client.FirebaseUser | undefined> => {
  if (!user) throw new Error('User is not defined');
  console.log('Retreiving firebase user');
  return <Client.FirebaseUser>(await usersCollection.doc(user.uid).get()).data();
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

export async function addNewDevice(firebaseDevice: Device.FirebaseDevice): Promise<void> {
  await deviceCollection.doc(firebaseDevice.uid).update(firebaseDevice);
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
export async function deleteDevice(device: Device.FirebaseDevice) {
  throw new Error('Not implemented');
}
