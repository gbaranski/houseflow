import React from 'react';
import 'react-toastify/dist/ReactToastify.css';
import LoginEmail from '../loginEmail';
import LoginGoogle from '../loginGoogle';

export default function LoginMethod() {
  return (
    <div>
      <LoginGoogle />
      <LoginEmail />
    </div>
  );
}
