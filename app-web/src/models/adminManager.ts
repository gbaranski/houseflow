import { useState } from 'react';
import { Device } from '@gbaranski/types';

export default () => {
  const [deviceModalVisible, setDeviceModalVisible] = useState<boolean>(false);
  const [deviceModalLoading, setDeviceModalLoading] = useState<boolean>(false);
  const [deviceModalFields, setDeviceModalFields] = useState({
    uid: '',
    secret: '',
    type: '',
  });

  const onDeviceAdd = (device: Device.FirebaseDevice) => {
    console.log({ device });
    setDeviceModalLoading(true);
    setTimeout(() => {
      setDeviceModalLoading(false);
      setDeviceModalVisible(false);
    }, 500);
  };

  const onDeviceCancel = () => {
    setDeviceModalVisible(false);
  };

  return {
    deviceModalVisible,
    setDeviceModalVisible,
    deviceModalLoading,
    setDeviceModalLoading,
    onDeviceAdd,
    onDeviceCancel,
    deviceModalFields,
    setDeviceModalFields,
  };
};
