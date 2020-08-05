import firebase from 'firebase/app';
import 'firebase/firestore';
import 'firebase/analytics';
import 'firebase/auth';
import 'firebase/app';

import { RequestHistory, TempHistory, FirebaseUser } from '@gbaranski/types';

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

export async function signInWithCredentials(email: string, password: string) {
  try {
    return firebase.auth().signInWithEmailAndPassword(email, password);
  } catch (e) {
    throw e;
  }
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

export async function convertFirebaseUser(
  userCredential: firebase.auth.UserCredential,
): Promise<FirebaseUser> {
  if (!userCredential.user) throw new Error('User is not defined');
  const usersDoc = await usersCollection.doc(userCredential.user.uid).get();
  if (!usersDoc.exists) throw new Error('User does not exist in database');
  const usersData = usersDoc.data() as FirebaseUser;
  const firebaseUser: FirebaseUser = {
    ...usersData,
  };
  return firebaseUser;
}
