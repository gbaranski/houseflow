import React from 'react';
import {
  DeviceType,
  AlarmclockData,
  WatermixerData,
  State,
} from '@gbaranski/types';
import { AnyDeviceData } from '@gbaranski/types/dist/other';

export interface ClientCurrentDevice<T extends DeviceType> {
  type: T;
  secret: string;
  uid: string;
  data: TDeviceData<T>;
  status: State;
}

export type TDeviceDataArgs<T extends AnyDeviceData | undefined> = {
  devicesData: T;
  setDevicesData: ((data: T) => any) | undefined;
};

type TDeviceData<
  T extends DeviceType | undefined
> = T extends DeviceType.ALARMCLOCK
  ? TDeviceDataArgs<AlarmclockData>
  : T extends DeviceType.WATERMIXER
  ? TDeviceDataArgs<WatermixerData>
  : undefined;

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
