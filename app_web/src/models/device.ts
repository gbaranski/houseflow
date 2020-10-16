import { useCallback, useReducer } from 'react';
import { Client, Device } from '@houseflow/types';
import { subscribeAllowedDevices } from '@/services/firebase';

type FirebaseDevicesAction = { type: 'UPDATE' | 'DELETE'; firebaseDevice: Device.FirebaseDevice };

const firebaseDevicesReducer = (
  firebaseDevices: Device.FirebaseDevice[],
  action: FirebaseDevicesAction,
) => {
  switch (action.type) {
    case 'UPDATE':
      return firebaseDevices
        .filter((_firebaseDevice) => _firebaseDevice.uid !== action.firebaseDevice.uid)
        .concat(action.firebaseDevice);
    case 'DELETE':
      return firebaseDevices.filter(
        (_firebaseDevice) => _firebaseDevice.uid !== action.firebaseDevice.uid,
      );
    default:
      throw new Error('Wrong action type');
  }
};
export default () => {
  const [firebaseDevices, dispatchFirebaseDevices] = useReducer(firebaseDevicesReducer, []);

  const onFirebaseDeviceUpdate = useCallback((newDevice: Device.FirebaseDevice) => {
    dispatchFirebaseDevices({ type: 'UPDATE', firebaseDevice: newDevice });
  }, []);

  const initializeFirebaseDevices = (firebaseUser: Client.FirebaseUser) => {
    subscribeAllowedDevices(firebaseUser, onFirebaseDeviceUpdate);
  };

  return {
    firebaseDevices,
    dispatchFirebaseDevices,
    initializeFirebaseDevices,
  };
};
