import React from 'react';
import { Device } from '@gbaranski/types';

interface IDeviceDataContext {
  devices: Device.ActiveDevice[];
  setDevices: ((devices: Device.ActiveDevice[]) => any) | undefined;
}

export const DeviceDataContext = React.createContext<IDeviceDataContext>({
  devices: [],
  setDevices: undefined,
});

interface DeviceDataProviderProps {
  children: React.ReactNode;
}

export const DeviceDataProvider = ({ children }: DeviceDataProviderProps) => {
  const [devices, setDevices] = React.useState<Device.ActiveDevice[]>([]);

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
