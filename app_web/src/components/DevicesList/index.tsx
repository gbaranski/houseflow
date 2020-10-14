import React from 'react';
import WatermixerCard from '@/components/Watermixer/card';
import DeviceCardSkeleton from '@/components/DeviceCardSkeleton';
import { Device, Watermixer, Alarmclock, Gate } from '@gbaranski/types';
import { Col } from 'antd';
import GateCard from '../Gate/card';

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
              <DeviceCardSkeleton key={device.uid} name={device.type} description="Device is offline" />
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
          case 'GATE': {
            return ( 
              <Col key={device.uid}>
                <GateCard
                  key={device.uid}
                  device={device as Device.FirebaseDevice<Gate.Data>}
                />
              </Col>
            )
          }
          default:
            return <DeviceCardSkeleton key={device.uid} name="Error" description={"Unrecognized device!"} />;
        }
      })}
    </>
  );
};

export default DeviceList;
