import React from 'react';
import { Device, AnyDeviceData } from '@gbaranski/types';

interface IDeviceDataContext {
  activeDevices: Device.ActiveDevice<AnyDeviceData>[];
  setActiveDevices:
    | ((devices: Device.ActiveDevice<AnyDeviceData>[]) => any)
    | undefined;
  firebaseDevices: Device.FirebaseDevice[];
  setFirebaseDevices: ((devices: Device.FirebaseDevice[]) => any) | undefined;
}

export const DeviceDataContext = React.createContext<IDeviceDataContext>({
  activeDevices: [],
  setActiveDevices: undefined,
  firebaseDevices: [],
  setFirebaseDevices: undefined,
});

interface DeviceDataProviderProps {
  children: React.ReactNode;
}

export const DeviceDataProvider = ({ children }: DeviceDataProviderProps) => {
  const [activeDevices, setActiveDevices] = React.useState<
    Device.ActiveDevice<AnyDeviceData>[]
  >([]);
  const [firebaseDevices, setFirebaseDevices] = React.useState<
    Device.FirebaseDevice[]
  >([]);

  return (
    <DeviceDataContext.Provider
      value={{
        activeDevices,
        setActiveDevices,
        firebaseDevices,
        setFirebaseDevices,
      }}
    >
      {children}
    </DeviceDataContext.Provider>
  );
};
