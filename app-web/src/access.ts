// src/access.ts
import { Client } from '@gbaranski/types';

export default function access(initialState: { firebaseUser?: Client.FirebaseUser }) {
  const { firebaseUser } = initialState || {};
  return {
    canAdmin: firebaseUser ? firebaseUser.role === 'admin' : false,
  };
}
