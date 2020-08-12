import React from 'react';
import { UserContext } from '../../providers/userProvider';
import { mdiGoogle } from '@mdi/js';
import { Button } from '@material-ui/core';
import Icon from '@mdi/react';
import {
  signToGoogleWithPopup,
  convertToFirebaseUser,
} from '../../services/firebase';
import { toast } from 'react-toastify';

export default function LoginGoogle() {
  const { setFirebaseUser } = React.useContext(UserContext);
  if (!setFirebaseUser) throw new Error('Cannot use context setFirebaseUser');

  const handleSubmit = async () => {
    try {
      const credentials = await signToGoogleWithPopup();
      if (!credentials.user) throw new Error('User cannot be null');
      const firebaseUser = await convertToFirebaseUser(credentials.user);
      if (!firebaseUser) throw new Error('Some error occured, cannot log in');
      setFirebaseUser(firebaseUser);
    } catch (e) {
      toast.error(e.message);
    }
  };

  return (
    <>
      <Button
        variant="outlined"
        color="primary"
        onClick={handleSubmit}
        fullWidth
        startIcon={<Icon path={mdiGoogle} size={1} />}
      >
        Sign in with Google
      </Button>
    </>
  );
}
