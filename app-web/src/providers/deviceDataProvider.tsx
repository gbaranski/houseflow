import React from 'react';
import { DeviceStatus, DeviceDataClient } from '@gbaranski/types';

interface IDeviceDataContext {
  deviceStatus: {
    setDeviceStatus: ((deviceStatus: DeviceStatus[]) => any) | undefined;
    deviceStatus: DeviceStatus[];
  };
  deviceData: {
    deviceData: DeviceDataClient[];
    setDeviceData: ((deviceData: DeviceDataClient[]) => any) | undefined;
  };
}

export const DeviceDataContext = React.createContext<IDeviceDataContext>({
  deviceStatus: {
    setDeviceStatus: undefined,
    deviceStatus: [],
  },
  deviceData: {
    setDeviceData: undefined,
    deviceData: [],
  },
});

interface DeviceDataProviderProps {
  children: React.ReactNode;
}

export const DeviceDataProvider = ({ children }: DeviceDataProviderProps) => {
  const [deviceStatus, setDeviceStatus] = React.useState<DeviceStatus[]>([]);
  const [deviceData, setDeviceData] = React.useState<DeviceDataClient[]>([]);
  return (
    <DeviceDataContext.Provider
      value={{
        deviceStatus: {
          deviceStatus,
          setDeviceStatus,
        },
        deviceData: {
          deviceData,
          setDeviceData,
        },
      }}
    >
      {children}
    </DeviceDataContext.Provider>
  );
};
