import React from 'react';
import LoginEmail from '@/components/loginEmail';
import LoginGoogle from '@/components/loginGoogle';
import LoginRegisterTemplate from '@/components/loginTemplate';

const Login: React.FC<{}> = () => {
  return (
    <LoginRegisterTemplate>
      <LoginGoogle />
      <LoginEmail />
    </LoginRegisterTemplate>
  );
};

export default Login;
