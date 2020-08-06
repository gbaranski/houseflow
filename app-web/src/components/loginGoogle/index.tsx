import React from 'react';
import { makeStyles } from '@material-ui/core/styles';
import { green } from '@material-ui/core/colors';
import { UserContext } from '../../providers/userProvider';
import { mdiGoogle } from '@mdi/js';
import { Button } from '@material-ui/core';
import Icon from '@mdi/react';
import {
  signToGoogleWithPopup,
  convertToFirebaseUser,
} from '../../services/firebase';
import { toast } from 'react-toastify';

const useStyles = makeStyles((theme) => ({
  submit: {
    margin: theme.spacing(1, 0, 2),
  },
  buttonProgress: {
    color: green[500],
    position: 'absolute',
    top: '50%',
    left: '50%',
    marginTop: -12,
    marginLeft: -12,
  },
  buttonSuccess: {
    backgroundColor: green[500],
    '&:hover': {
      backgroundColor: green[700],
    },
  },
  wrapper: {
    margin: theme.spacing(1),
    position: 'relative',
  },
}));

export default function LoginGoogle() {
  const classes = useStyles();

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
