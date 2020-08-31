import { useState } from 'react';
import { Device, Client, CurrentConnections } from '@gbaranski/types';
import { setupOnOpenListeners } from '@/services/websocket';
import { getAllowedDevices, getAllDevices } from '@/services/firebase';

export default () => {
  const [activeDevices, setActiveDevices] = useState<Device.ActiveDevice[]>([]);
  const [firebaseDevices, setFirebaseDevices] = useState<Device.FirebaseDevice[]>([]);
  const [allConnections, setAllConnections] = useState<CurrentConnections>();
  const [allFirebaseDevices, setAllFirebaseDevices] = useState<Device.FirebaseDevice[]>([]);

  const onMessage = (message: MessageEvent) => {
    const response = JSON.parse(message.data) as Client.Response;
    if (response.requestType === 'DATA') {
      console.log('Received new data', response.data);
      setActiveDevices(response.data as Device.ActiveDevice[]);
    } else if (response.requestType === 'CONNECTIONS') {
      console.log('Received connections', response.data);
      setAllConnections(response.data as CurrentConnections);
    }
  };

  const setupListeners = () => {
    setupOnOpenListeners(onMessage);
  };

  const getAndSetFirebaseDevices = async (firebaseUser: Client.FirebaseUser | undefined) => {
    if (!firebaseUser) throw new Error('Firebase user is not defined');
    if (firebaseDevices.length > 1) return;
    const allowedDevices = await getAllowedDevices(firebaseUser);
    setFirebaseDevices(allowedDevices);
  };

  const getAndSetAllDevices = async () => {
    console.log('Attempting to get and set all devices');
    return setAllFirebaseDevices(await getAllDevices());
  };

  return {
    activeDevices,
    setActiveDevices,
    firebaseDevices,
    setFirebaseDevices,
    setupListeners,
    getAndSetFirebaseDevices,
    allConnections,
    allFirebaseDevices,
    setAllFirebaseDevices,
    getAndSetAllDevices,
  };
};
