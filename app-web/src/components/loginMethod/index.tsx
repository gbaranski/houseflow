import React from 'react';
import 'react-toastify/dist/ReactToastify.css';
import { makeStyles } from '@material-ui/core';
import LoginEmail from '../loginEmail';

const useStyles = makeStyles((theme) => ({
  providerButton: {
    display: 'inline-flex',
    alignItems: 'center',
    width: '95%',
    margin: 8,
    padding: 0,
    fontWeight: 500,
    fontSize: 14,
    border: 1,
    borderRadius: 2,
    cursor: 'pointer',
  },
}));

export default function LoginMethod() {
  const styles = useStyles();

  return (
    <div>
      <LoginEmail />
    </div>
  );
}
