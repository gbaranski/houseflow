import React from 'react';
import { FirebaseUser } from '@gbaranski/types';

interface IUserContext {
  firebaseUser: FirebaseUser | undefined;
  setFirebaseUser: ((firebaseUser: FirebaseUser) => any) | undefined;
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
    FirebaseUser | undefined
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
