import { useState } from 'react';
import { Client, Device } from '@gbaranski/types';
import { subscribeAllowedDevices } from '@/services/firebase';

export default () => {
  const [firebaseDevices, setFirebaseDevices] = useState<Device.FirebaseDevice[]>([]);

  const onFirebaseDeviceUpdate = (newDevice: Device.FirebaseDevice) => {
    const newFirebaseDevices = firebaseDevices
      .filter((device) => device.uid !== newDevice.uid)
      .concat(newDevice);
    setFirebaseDevices(newFirebaseDevices);
  };

  const initializeFirebaseDevices = (firebaseUser: Client.FirebaseUser) => {
    subscribeAllowedDevices(firebaseUser, onFirebaseDeviceUpdate);
  };

  return {
    firebaseDevices,
    setFirebaseDevices,
    initializeFirebaseDevices,
  };
};
