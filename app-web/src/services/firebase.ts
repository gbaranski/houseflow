import firebase, { User } from 'firebase/app';
import 'firebase/firestore';
import 'firebase/analytics';
import 'firebase/auth';

import {
  RequestHistory,
  TempHistory,
  Client,
  Device,
  Alarmclock,
  Watermixer,
  AnyDeviceData,
} from '@gbaranski/types';

const firebaseConfig = {
  apiKey: 'AIzaSyCpRLmvfBf-SpwkDUHKa_vrbEeIvSzHNOY',
  authDomain: 'controlhome-7bbcc.firebaseapp.com',
  databaseURL: 'https://controlhome-7bbcc.firebaseio.com',
  projectId: 'controlhome-7bbcc',
  storageBucket: 'controlhome-7bbcc.appspot.com',
  messagingSenderId: '794654805763',
  appId: '1:794654805763:web:9178272307d06e5eade336',
  measurementId: 'G-J8271YJZER',
};

const app = firebase.initializeApp(firebaseConfig);
firebase.analytics();

const db = firebase.firestore();
const requestCollection = db.collection('requests');
const tempHistoryCollection = db.collection('temp-history');
const usersCollection = db.collection('users');

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

export async function getRequestHistory() {
  const requestHistory: RequestHistory[] = [];
  (await requestCollection.get()).forEach((doc) => {
    const docData: RequestHistory = doc.data() as RequestHistory;
    requestHistory.push(docData);
  });
  return requestHistory;
}

export async function getTempHistory() {
  const tempHistory: TempHistory[] = [];
  (await tempHistoryCollection.get()).forEach((doc) => {
    const docData: TempHistory = doc.data() as TempHistory;
    tempHistory.push(docData);
  });
  console.log(tempHistory);
  return tempHistory;
}

export async function convertToFirebaseUser(user: firebase.User): Promise<Client.FirebaseUser> {
  if (!user) throw new Error('User is not defined');
  const usersDoc = await usersCollection.doc(user.uid).get();
  if (!usersDoc.exists) throw new Error('User does not exist in database');
  const usersData = usersDoc.data() as Client.FirebaseUser;
  const firebaseUser: Client.FirebaseUser = {
    ...usersData,
  };
  return firebaseUser;
}

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
  const currentDevices = firebaseUser.devices.full_access.map(async (doc) => {
    const docSnapshot = await doc.get();
    const docData = docSnapshot.data();
    if (!docData) throw new Error('Document data is not defined');

    const parsedDocData = docData as Device.FirebaseDevice;
    if (!parsedDocData.type) throw new Error('Type od allowed device not defined');
    const currentDevice: Device.FirebaseDevice = {
      uid: parsedDocData.uid,
      type: parsedDocData.type,
    };
    return currentDevice;
  });

  const resolvedDevices = await Promise.all(currentDevices);

  return resolvedDevices;
}

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
