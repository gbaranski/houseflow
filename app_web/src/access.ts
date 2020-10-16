// src/access.ts
import { Client } from '@houseflow/types';

export default function access(initialState: { firebaseUser?: Client.FirebaseUser }) {
  const { firebaseUser } = initialState || {};
  return {
    canAdmin: firebaseUser ? firebaseUser.role === 'admin' : false,
  };
}
