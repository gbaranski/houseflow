import React from 'react';
import { sendPasswordResetEmail, signInWithCredentials } from '@/services/firebase';
import { message, Input, Button, Form, Popconfirm } from 'antd';
import { history, useModel } from 'umi';
import { useForm } from 'antd/lib/form/Form';

export default function LoginEmail() {
  const initialState = useModel('@@initialState');
  const [form] = useForm();

  const onPasswordReset = async () => {
    try {
      await form.validateFields(['email']);
      const email = form.getFieldValue('email');
      await sendPasswordResetEmail(email);
      message.success('Sent email! Check your inbox');
    } catch (e) {
      if (!e.message) return message.error('Please input your email');
      message.error(e.message);
    }
  };

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
      name="loginForm"
      initialValues={{ remember: true }}
      onFinish={onFinish}
      form={form}
      className="login-form"
    >
      <Form.Item
        name="email"
        rules={[{ required: true, message: 'Please input your email!', type: 'email' }]}
      >
        <Input placeholder="email" />
      </Form.Item>

      <Form.Item
        noStyle
        name="password"
        rules={[{ required: true, message: 'Please input your password!', type: 'string' }]}
      >
        <Input.Password placeholder="password" />
      </Form.Item>
      <Form.Item>
        <Popconfirm
          title="Are you sure you want to reset password?"
          onConfirm={onPasswordReset}
          okText="Yes"
          cancelText="No"
        >
          <a href="#">Forgot password?</a>
        </Popconfirm>
      </Form.Item>

      <Form.Item style={{}}>
        <Button type="primary" htmlType="submit" className="login-form-button">
          Submit
        </Button>
      </Form.Item>
    </Form>
  );
}
