import React, { useState } from 'react';
import { Button, Modal, Form, Input, message } from 'antd';
import { Device } from '@gbaranski/types';
import { addNewDevice } from '@/services/firebase';

interface AddDeviceFormProps {
  visible: boolean;
  setVisible: (state: boolean) => any;
}

const AddDeviceForm: React.FC<AddDeviceFormProps> = ({ visible, setVisible }) => {
  const [form] = Form.useForm();
  const [loading, setLoading] = useState<boolean>(false);

  const onCreate = async (values: Device.FirebaseDevice) => {
    try {
      setLoading(true);
      await addNewDevice(values);
      setLoading(false);
      setVisible(false);
      message.info('Success!');
    } catch (e) {
      message.error(e.message);
      console.log(e);
      setLoading(false);
    }
  };

  return (
    <Modal
      visible={visible}
      confirmLoading={loading}
      title="Add new device"
      okText="Add"
      cancelText="Cancel"
      onCancel={() => setVisible(false)}
      onOk={() => {
        try {
          form.validateFields().then((values) => onCreate(values as Device.FirebaseDevice));
        } catch (info) {
          console.log('Validate Failed:', info);
        }
      }}
    >
      <Form form={form} layout="vertical" name="add_device">
        <Form.Item
          name="uid"
          label="Device UID"
          rules={[{ required: true, message: 'Please input the device UID!' }]}
        >
          <Input />
        </Form.Item>
        <Form.Item
          name="secret"
          label="Device Secret"
          rules={[{ required: true, message: 'Please input the device secret!' }]}
        >
          <Input type="textarea" />
        </Form.Item>
        <Form.Item
          name="type"
          label="Device Type"
          rules={[{ required: true, message: 'Please input the device type!' }]}
        >
          <Input type="textarea" />
        </Form.Item>
      </Form>
    </Modal>
  );
};

export default () => {
  const [visible, setVisible] = useState(false);

  return (
    <>
      <AddDeviceForm visible={visible} setVisible={setVisible} />
      <Button
        type="primary"
        onClick={() => {
          setVisible(true);
        }}
      >
        Add new Device
      </Button>
    </>
  );
};
