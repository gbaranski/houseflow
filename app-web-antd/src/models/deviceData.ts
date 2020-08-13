import { useState } from 'react';
import { Device, AnyDeviceData, Client } from '@gbaranski/types';
import { setupOnOpenListeners } from '@/services/websocket';
import { getAllowedDevices } from '@/services/firebase';

export default () => {
  const [activeDevices, setActiveDevices] = useState<Device.ActiveDevice<AnyDeviceData>[]>([]);
  const [firebaseDevices, setFirebaseDevices] = useState<Device.FirebaseDevice[]>([]);

  const onMessage = (message: MessageEvent) => {
    const response = JSON.parse(message.data) as Client.Response;
    if (response.requestType === 'DATA') {
      console.log('Received new data', response.data);
      setActiveDevices(response.data as Device.ActiveDevice<AnyDeviceData>[]);
    }
  };

  const setupListeners = () => {
    setupOnOpenListeners(onMessage);
  };

  const getAndSetFirebaseDevices = (firebaseUser: Client.FirebaseUser | undefined) => {
    if (!firebaseUser) throw new Error('Firebase user is not defined');
    getAllowedDevices(firebaseUser).then((allowedDevices) => setFirebaseDevices(allowedDevices));
  };

  return {
    activeDevices,
    setActiveDevices,
    firebaseDevices,
    setFirebaseDevices,
    setupListeners,
    getAndSetFirebaseDevices,
  };
};
