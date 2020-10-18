import React, { FC } from 'react';
import WatermixerCard from '@/components/Watermixer/card';
import GarageCard from '@/components/Garage/card';
import DeviceCardSkeleton from '@/components/DeviceCardSkeleton';
import { Device } from '@houseflow/types';
import { Col } from 'antd';
import GateCard from '../Gate/card';

interface UiDevice {
  type: Device.DeviceType;
  component: FC<any>;
}

const uiDevices: UiDevice[] = [
  {
    type: 'WATERMIXER',
    component: WatermixerCard,
  },
  {
    type: 'GATE',
    component: GateCard,
  },
  {
    type: 'GARAGE',
    component: GarageCard,
  },
];

interface DeviceListProps {
  firebaseDevices: Device.FirebaseDevice[];
}
const DeviceList: React.FC<DeviceListProps> = ({ firebaseDevices }) => {
  return (
    <>
      {firebaseDevices
        .sort((deviceA, deviceB) => (deviceA.status ? deviceA.uid.localeCompare(deviceB.uid) : 1))
        .map((device) => {
          if (!device.status)
            return (
              <Col key={device.uid}>
                <DeviceCardSkeleton
                  key={device.uid}
                  name={device.type}
                  description="Device is offline"
                />
              </Col>
            );

          const uiDevice = uiDevices.find((_uiDevice) => _uiDevice.type === device.type);
          if (!uiDevice) {
            return (
              <DeviceCardSkeleton
                key={device.uid}
                name="Error"
                description="Unrecognized device!"
              />
            );
          }
          return (
            <Col key={device.uid}>
              <uiDevice.component key={device.uid} device={device} />
            </Col>
          );
        })}
    </>
  );
};

export default DeviceList;
