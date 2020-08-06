import React, { useContext } from 'react';
import Button from '@material-ui/core/Button';
import TextField from '@material-ui/core/TextField';
import { makeStyles } from '@material-ui/core/styles';
import { toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';
import {
  signInWithCredentials,
  convertToFirebaseUser,
} from '../../services/firebase';
import { CircularProgress } from '@material-ui/core';
import { green } from '@material-ui/core/colors';
import { UserContext } from '../../providers/userProvider';

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

export default function LoginEmail() {
  const classes = useStyles();

  const { setFirebaseUser } = React.useContext(UserContext);
  if (!setFirebaseUser) throw new Error('Cannot use context setFirebaseUser');

  const [email, setEmail] = React.useState('');
  const [password, setPassword] = React.useState('');
  const [loading, setLoading] = React.useState(false);

  const handleSubmit = async (e: any) => {
    e.preventDefault();
    if (loading) toast.error("Dont't press during sending!");
    setLoading(true);
    try {
      const credentials = await signInWithCredentials(email, password);
      if (!credentials.user) throw new Error('User cannot be null');
      const firebaseUser = await convertToFirebaseUser(credentials.user);
      if (!firebaseUser) throw new Error('Some error occured, cannot log in');
      setFirebaseUser(firebaseUser);
    } catch (e) {
      toast.error(e.message);
    }
    setLoading(false);
  };

  return (
    <>
      <form autoComplete="off" onSubmit={handleSubmit}>
        <TextField
          variant="outlined"
          margin="normal"
          required
          fullWidth
          id="email"
          label="Email"
          name="email"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          autoFocus
        />
        <TextField
          variant="outlined"
          margin="normal"
          required
          fullWidth
          name="password"
          label="Password"
          type="password"
          id="password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
        />
        <div className={classes.wrapper}>
          <Button
            type="submit"
            fullWidth
            variant="contained"
            color="primary"
            className={classes.submit}
            disabled={loading}
            onClick={handleSubmit}
          >
            Sign in
          </Button>
          {loading && (
            <CircularProgress size={24} className={classes.buttonProgress} />
          )}
        </div>
      </form>
    </>
  );
}
