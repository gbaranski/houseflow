import React from 'react';
import WatermixerCard from '@/components/Watermixer/card';
import DeviceCardSkeleton from '@/components/DeviceCardSkeleton';
import { Device, Watermixer, Alarmclock } from '@gbaranski/types';
import { Col } from 'antd';
import AlarmclockCard from '../Alarmclock/card';

interface DeviceListProps {
  firebaseDevices: Device.FirebaseDevice[];
}
const DeviceList: React.FC<DeviceListProps> = ({ firebaseDevices }) => {
  return (
    <>
      {firebaseDevices.map((device) => {
        if (!device.status)
          return (
            <Col key={device.uid}>
              <DeviceCardSkeleton key={device.uid} name={device.type} />
            </Col>
          );
        switch (device.type) {
          case 'WATERMIXER':
            return (
              <Col key={device.uid}>
                <WatermixerCard
                  key={device.uid}
                  device={device as Device.FirebaseDevice<Watermixer.Data>}
                />
              </Col>
            );
          case 'ALARMCLOCK':
            return (
              <Col key={device.uid}>
                <AlarmclockCard
                  key={device.uid}
                  device={device as Device.FirebaseDevice<Alarmclock.Data>}
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
