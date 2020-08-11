import React from 'react';
import CircularProgress from '@material-ui/core/CircularProgress';
import Title from '../../components/title';

type TLoadingPage = { title: string };

const LoadingPage = ({ title }: TLoadingPage) => {
  return (
    <div
      style={{
        position: 'absolute',
        top: '50%',
        left: '50%',
        textAlign: 'center',
        transform: 'translate(-50%, -50%)',
      }}
    >
      <CircularProgress />
      <Title>Loading</Title>
    </div>
  );
};

export default LoadingPage;
