import React from 'react';
import LoginEmail from '@/components/LoginMethods/loginEmail';
import LoginGoogle from '@/components/LoginMethods/loginGoogle';
import LoginRegisterTemplate from '@/components/loginTemplate';
import LoginApple from '@/components/LoginMethods/loginApple';

const Login: React.FC<{}> = () => {
  return (
    <LoginRegisterTemplate>
      <LoginGoogle />
      <LoginApple />
      <LoginEmail />
    </LoginRegisterTemplate>
  );
};

export default Login;
