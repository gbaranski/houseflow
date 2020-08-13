import React from 'react';
import WatermixerCard from '@/components/Watermixer/card';
import DeviceCardSkeleton from '@/components/DeviceCardSkeleton';
import { Device, Watermixer, AnyDeviceData } from '@gbaranski/types';
import { Col } from 'antd';

interface DeviceListProps {
  activeDevices: Device.ActiveDevice<AnyDeviceData>[];
  firebaseDevices: Device.FirebaseDevice[];
}
const DeviceList: React.FC<DeviceListProps> = ({ firebaseDevices, activeDevices }) => {
  return (
    <>
      {firebaseDevices.map((device) => {
        const activeDevice = activeDevices.find((_device) => _device.uid === device.uid);
        if (!activeDevice)
          return (
            <Col>
              <DeviceCardSkeleton key={device.uid} name={device.type} />
            </Col>
          );
        switch (activeDevice.type) {
          case 'WATERMIXER':
            return (
              <Col>
                <WatermixerCard
                  key={activeDevice.uid}
                  device={activeDevice as Device.ActiveDevice<Watermixer.Data>}
                />
              </Col>
            );
          default:
            return <DeviceCardSkeleton key={device.uid} name="Error" />;
        }
      })}
    </>
  );
};

export default DeviceList;
