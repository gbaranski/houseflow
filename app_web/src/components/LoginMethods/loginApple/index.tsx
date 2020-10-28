import React from 'react';
import { Button, message } from 'antd';
import { history, useModel } from 'umi';
import { AppleOutlined } from '@ant-design/icons';
import { signInViaAppleWithPopup } from '@/services/firebase';

export default function LoginApple({ register }: { register?: boolean }) {
  const initialState = useModel('@@initialState');
  const handleSubmit = async () => {
    try {
      const credentials = await signInViaAppleWithPopup();
      if (!credentials.user) throw new Error('Current user not defined');
      message.info('User logged in');
      history.replace('/welcome');
      initialState.refresh();
    } catch (e) {
      message.error(e.message);
    }
  };
  return (
    <Button onClick={handleSubmit} icon={<AppleOutlined />} block style={{ marginBottom: 10 }}>
      {register ? 'Register' : 'Sign in'} with Apple
    </Button>
  );
}
