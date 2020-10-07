import React, { useState } from 'react';
import {
  registerWithCredentials,
  sendPasswordResetEmail,
  signInWithCredentials,
} from '@/services/firebase';
import { message, Input, Button, Form, Popconfirm } from 'antd';
import { useHistory, useModel } from 'umi';
import { useForm } from 'antd/lib/form/Form';
import { Rule } from 'antd/lib/form';

const passwordValidationRules: Rule[] = [
  { required: true, message: 'Please input your password!', type: 'string' },
  { min: 8, message: 'Your password needs to be atleast 8 characters long' },
];

interface FormSubmitValues {
  email: string;
  password: string;
}

export default function LoginEmail({ register }: { register?: boolean }) {
  const initialState = useModel('@@initialState');
  const [buttonLoading, setButtonLoading] = useState(false);
  const history = useHistory();
  const [form] = useForm();

  const onPasswordReset = async () => {
    try {
      await form.validateFields(['email']);
      const email = form.getFieldValue('email');
      await sendPasswordResetEmail(email);
      message.success('Sent email! Check your inbox');
    } catch (e) {
      if (!e.message) {
        message.error('Please input your email');
        return;
      }
      message.error(e.message);
    }
  };

  const onFinish = async (values: FormSubmitValues): Promise<void> => {
    try {
      setButtonLoading(true);
      const submitFunc = register ? registerWithCredentials : signInWithCredentials;
      const credentials = await submitFunc(values.email, values.password);
      if (!credentials) throw new Error('Credentials are not defined');
      message.info('User logged in');
      history.replace('/welcome');
      initialState.refresh();
    } catch (e) {
      message.error(e.message);
      console.log(e);
    } finally {
      setButtonLoading(false);
    }
  };

  return (
    <Form name="loginForm" onFinish={onFinish} form={form} className="login-form">
      <Form.Item
        style={{ marginBottom: 12 }}
        name="email"
        hasFeedback
        rules={[{ required: true, message: 'Please input your email!', type: 'email' }]}
      >
        <Input placeholder="Email" />
      </Form.Item>

      <Form.Item
        style={{ marginBottom: 12 }}
        name="password"
        hasFeedback
        rules={passwordValidationRules}
      >
        <Input.Password placeholder="Password" />
      </Form.Item>

      {register && (
        <Form.Item
          style={{ marginBottom: 12 }}
          name="confirmPassword"
          hasFeedback
          dependencies={['password']}
          rules={[
            ...passwordValidationRules,
            ({ getFieldValue }) => ({
              validator(rule, value) {
                if (!value || getFieldValue('password') === value) {
                  return Promise.resolve();
                }
                return Promise.reject('The two passwords that you entered do not match!');
              },
            }),
          ]}
        >
          <Input.Password placeholder="Confirm password" />
        </Form.Item>
      )}

      <Form.Item style={{ marginBottom: -10 }}>
        {!register && (
          <Form.Item style={{ float: 'left' }}>
            <Popconfirm
              title="Are you sure you want to reset password?"
              onConfirm={onPasswordReset}
              okText="Yes"
              cancelText="No"
            >
              <a href="#">Forgot password?</a>
            </Popconfirm>
          </Form.Item>
        )}

        <Form.Item style={{ float: register ? 'left' : 'right' }}>
          {register ? (
            <a onClick={() => history.replace('/user/login')}>Already have account</a>
          ) : (
            <a onClick={() => history.replace('/user/register')}>Create account</a>
          )}
        </Form.Item>
      </Form.Item>

      <Form.Item>
        <Button
          loading={buttonLoading}
          type="primary"
          htmlType="submit"
          className="login-form-button"
          style={{ width: '100%' }}
        >
          {register ? 'Register' : 'Log in'}
        </Button>
      </Form.Item>
    </Form>
  );
}
