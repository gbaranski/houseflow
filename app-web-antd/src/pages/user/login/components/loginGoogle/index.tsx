import React from 'react';
import { Button, message } from 'antd';
import { history } from 'umi';
import { GoogleOutlined } from '@ant-design/icons';
import { signToGoogleWithPopup } from '@/services/firebase';

export default function LoginGoogle() {
  const handleSubmit = async () => {
    try {
      const credentials = await signToGoogleWithPopup();
      if (!credentials.user) throw new Error('Current user not defined');
      message.info('User logged in');
      history.replace('/welcome');
    } catch (e) {
      message.error(e.message);
    }
  };
  return (
    <Button onClick={handleSubmit} icon={<GoogleOutlined />} block style={{ marginBottom: 10 }}>
      Sign in with Google
    </Button>
  );
}
