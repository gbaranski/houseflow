import firebase from 'firebase/app';

const firebaseConfig = {
  apiKey: 'AIzaSyDEB-EPW4vdhqBGFOI5LQAo-CD3ZCl6d4s',
  authDomain: 'houseflow-prod.firebaseapp.com',
  databaseURL:
    'https://houseflow-prod-default-rtdb.europe-west1.firebasedatabase.app',
  projectId: 'houseflow-prod',
  storageBucket: 'houseflow-prod.appspot.com',
  messagingSenderId: '77077316480',
  appId: '1:77077316480:web:da441913520080f9e7b765',
  measurementId: 'G-Z909M4973Z',
};

const app = firebase.initializeApp(firebaseConfig);
const auth = app.auth();
const googleProvider = new firebase.auth.GoogleAuthProvider();

export const signInWithGoogle = async (): Promise<firebase.auth.UserCredential> => {
  return auth.signInWithPopup(googleProvider);
};
