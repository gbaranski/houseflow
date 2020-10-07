import LoginRegisterTemplate from '@/components/loginTemplate';
import { initializeNewUser } from '@/services/firebase';
import { Button, Form, Input, message } from 'antd';
import React, { useState } from 'react';
import { useModel, useHistory } from 'umi';

const UserWizard = () => {
  const history = useHistory();

  const initialState = useModel('@@initialState');

  const { currentUser } = initialState.initialState || {};
  const [buttonLoading, setButtonLoading] = useState(false);

  if (!currentUser) return history.push('/user/login');

  const onFinish = async (values: { username: string }) => {
    console.log({ values });
    try {
      setButtonLoading(true);
      await initializeNewUser({
        username: values.username,
      });
      message.success('Success initializing user!');
      initialState.refresh();
    } catch (e) {
      message.error(`${e.code} - ${e.message}`);
      console.log(e);
    } finally {
      setButtonLoading(false);
    }
  };
  return (
    <LoginRegisterTemplate>
      <Form
        name="loginForm"
        initialValues={{ username: currentUser.displayName }}
        className="login-form"
        onFinish={onFinish}
      >
        <Form.Item
          style={{ marginBottom: 12 }}
          name="username"
          hasFeedback
          rules={[{ required: true, message: 'Please input your username!', type: 'string' }]}
        >
          <Input placeholder="Username" />
        </Form.Item>

        <Form.Item>
          <Button
            loading={buttonLoading}
            type="primary"
            htmlType="submit"
            className="login-form-button"
            style={{ width: '100%' }}
          >
            Finish setup
          </Button>
        </Form.Item>
      </Form>
    </LoginRegisterTemplate>
  );
};

export default UserWizard;
