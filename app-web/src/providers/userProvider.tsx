import React from 'react';
import { Client } from '@gbaranski/types';

interface IUserContext {
  firebaseUser: Client.FirebaseUser | undefined;
  setFirebaseUser: ((firebaseUser: Client.FirebaseUser) => any) | undefined;
}

export const UserContext = React.createContext<IUserContext>({
  firebaseUser: undefined,
  setFirebaseUser: undefined,
});

interface UserProviderProps {
  children: React.ReactNode;
}

export const UserProvider = ({ children }: UserProviderProps) => {
  const [firebaseUser, setFirebaseUser] = React.useState<
    Client.FirebaseUser | undefined
  >();
  return (
    <UserContext.Provider
      value={{
        firebaseUser,
        setFirebaseUser,
      }}
    >
      {children}
    </UserContext.Provider>
  );
};
