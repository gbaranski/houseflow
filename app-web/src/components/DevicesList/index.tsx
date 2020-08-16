import React from 'react';
import WatermixerCard from '@/components/Watermixer/card';
import DeviceCardSkeleton from '@/components/DeviceCardSkeleton';
import { Device, Watermixer, AnyDeviceData, Alarmclock } from '@gbaranski/types';
import { Col } from 'antd';
import AlarmclockCard from '../Alarmclock/card';

interface DeviceListProps {
  activeDevices: Device.ActiveDevice[];
  firebaseDevices: Device.FirebaseDevice[];
}
const DeviceList: React.FC<DeviceListProps> = ({ firebaseDevices, activeDevices }) => {
  return (
    <>
      {firebaseDevices.map((device) => {
        const activeDevice = activeDevices.find((_device) => _device.uid === device.uid);
        if (!activeDevice)
          return (
            <Col key={device.uid}>
              <DeviceCardSkeleton key={device.uid} name={device.type} />
            </Col>
          );
        switch (activeDevice.type) {
          case 'WATERMIXER':
            return (
              <Col key={device.uid}>
                <WatermixerCard
                  key={activeDevice.uid}
                  device={activeDevice as Device.ActiveDevice<Watermixer.Data>}
                />
              </Col>
            );
          case 'ALARMCLOCK':
            return (
              <Col key={device.uid}>
                <AlarmclockCard
                  key={activeDevice.uid}
                  device={activeDevice as Device.ActiveDevice<Alarmclock.Data>}
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
