import React from 'react';
import {
  DeviceType,
  AlarmclockData,
  WatermixerData,
  State,
  ClientCurrentDevice,
} from '@gbaranski/types';

interface IDeviceDataContext {
  devices: ClientCurrentDevice<DeviceType>[];
  setDevices: ((devices: ClientCurrentDevice<DeviceType>[]) => any) | undefined;
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
    ClientCurrentDevice<DeviceType>[]
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
