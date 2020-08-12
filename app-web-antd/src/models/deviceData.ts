import { useState } from 'react';
import { Device, AnyDeviceData } from '@gbaranski/types';

export default () => {
  const [activeDevices, setActiveDevices] = useState<Device.ActiveDevice<AnyDeviceData>[]>([]);
  const [firebaseDevices, setFirebaseDevices] = useState<Device.FirebaseDevice[]>([]);

  return {
    activeDevices,
    setActiveDevices,
    firebaseDevices,
    setFirebaseDevices,
  };
};
