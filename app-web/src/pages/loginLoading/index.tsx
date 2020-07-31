import React from 'react';
import CircularProgress from '@material-ui/core/CircularProgress';
import Title from '../../components/title';

function LoginLoading() {
  return (
    <div
      style={{
        position: 'absolute',
        top: '50%',
        left: '50%',
        textAlign: 'center',
        transform: 'translate(-50%, -50%)',
      }}>
      <CircularProgress />
      <Title>Loading</Title>
    </div>
  );
}

export default LoginLoading;
