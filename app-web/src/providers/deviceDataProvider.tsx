import React from 'react';
import { Device, AnyDeviceData } from '@gbaranski/types';

interface IDeviceDataContext {
  devices: Device.ActiveDevice<AnyDeviceData>[];
  setDevices:
    | ((devices: Device.ActiveDevice<AnyDeviceData>[]) => any)
    | undefined;
}

export const DeviceDataContext = React.createContext<IDeviceDataContext>({
  devices: [],
  setDevices: undefined,
});

interface DeviceDataProviderProps {
  children: React.ReactNode;
}

export const DeviceDataProvider = ({ children }: DeviceDataProviderProps) => {
  const [devices, setDevices] = React.useState<
    Device.ActiveDevice<AnyDeviceData>[]
  >([]);

  return (
    <DeviceDataContext.Provider
      value={{
        devices,
        setDevices,
      }}
    >
      {children}
    </DeviceDataContext.Provider>
  );
};
