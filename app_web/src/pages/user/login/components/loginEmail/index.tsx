import React from 'react';
import { signInWithCredentials } from '@/services/firebase';
import { message, Input, Button, Form } from 'antd';
import { history, useModel } from 'umi';

export default function LoginEmail() {
  const initialState = useModel('@@initialState');
  const onFinish = async (values: {
    email: string;
    password: string;
    remember: boolean;
  }): Promise<void> => {
    try {
      const credentials = await signInWithCredentials(values.email, values.password);
      if (!credentials.user) throw new Error('Current user not defined');
      message.info('User logged in');
      history.replace('/welcome');
      initialState.refresh();
    } catch (e) {
      message.error(e.message);
      console.log(e);
    }
  };

  return (
    <Form
      name="normal_login"
      initialValues={{ remember: true }}
      // @ts-ignore
      onFinish={onFinish}
      className="login-form"
    >
      <Form.Item
        name="email"
        rules={[{ required: true, message: 'Please input your email!', type: 'email' }]}
      >
        <Input placeholder="email" />
      </Form.Item>

      <Form.Item
        name="password"
        rules={[{ required: true, message: 'Please input your password!', type: 'string' }]}
      >
        <Input.Password placeholder="password" />
      </Form.Item>

      {/* <Form.Item name="remember" valuePropName="checked" noStyle>
        <Checkbox>Remember me</Checkbox>
      </Form.Item> */}

      <Form.Item style={{}}>
        <Button type="primary" htmlType="submit" className="login-form-button">
          Submit
        </Button>
      </Form.Item>
    </Form>
  );
}
