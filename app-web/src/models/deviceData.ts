import { useState } from 'react';
import { Device, Client, CurrentConnections } from '@gbaranski/types';
import { getAllowedDevices, getAllDevices } from '@/services/firebase';

export default () => {
  const [activeDevices, setActiveDevices] = useState<Device.ActiveDevice[]>([]);
  const [firebaseDevices, setFirebaseDevices] = useState<Device.FirebaseDevice[]>([]);
  const [allConnections, setAllConnections] = useState<CurrentConnections>();
  const [allFirebaseDevices, setAllFirebaseDevices] = useState<Device.FirebaseDevice[]>([]);

  const setDataListeners = (socket: SocketIOClient.Socket) => {
    socket.on('device_update', (data: string) => {
      console.log(`Received device_update ${data}`);
      const activeDevice: Device.ActiveDevice = JSON.parse(data);
      const doesAlreadyExist = activeDevices.some((device) => device.uid === activeDevice.uid);
      if (doesAlreadyExist) {
        setActiveDevices(
          activeDevices.map((device) => (device.uid === activeDevice.uid ? activeDevice : device)),
        );
      } else {
        setActiveDevices(activeDevices.concat(activeDevice));
      }
    });
  };
  const getActiveDevices = (socket: SocketIOClient.Socket) => {
    socket.emit('get_active_devices', (data: string) => {
      console.log({ deviceData: JSON.parse(data) });
      setActiveDevices(JSON.parse(data));
    });
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
    setDataListeners,
    getActiveDevices,
    activeDevices,
    setActiveDevices,
    firebaseDevices,
    setFirebaseDevices,
    getAndSetFirebaseDevices,
    allConnections,
    allFirebaseDevices,
    setAllFirebaseDevices,
    getAndSetAllDevices,
  };
};
