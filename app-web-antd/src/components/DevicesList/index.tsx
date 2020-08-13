import React from 'react';
import WatermixerCard from '@/components/Watermixer/card';
import DeviceCardSkeleton from '@/components/DeviceCardSkeleton';
import { Device, Watermixer, AnyDeviceData } from '@gbaranski/types';

interface DeviceListProps {
  activeDevices: Device.ActiveDevice<AnyDeviceData>[];
  firebaseDevices: Device.FirebaseDevice[];
}
const DeviceList: React.FC<DeviceListProps> = ({ firebaseDevices, activeDevices }) => {
  return (
    <>
      {firebaseDevices.map((device) => {
        const activeDevice = activeDevices.find((_device) => _device.uid === device.uid);
        if (!activeDevice) return <DeviceCardSkeleton key={Math.random()} name={device.type} />;
        switch (activeDevice.type) {
          case 'WATERMIXER':
            return (
              <WatermixerCard
                key={activeDevice.uid}
                device={activeDevice as Device.ActiveDevice<Watermixer.Data>}
              />
            );
          default:
            return <DeviceCardSkeleton key={Math.random()} name="Error" />;
        }
      })}
    </>
  );
};

export default DeviceList;
