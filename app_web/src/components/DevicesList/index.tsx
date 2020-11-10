import React from 'react';
import { Device } from '@houseflow/types';
import { Row } from 'antd';
import DeviceCard from '../DeviceCard';

interface DeviceListProps {
  firebaseDevices: Device.FirebaseDevice[];
}
const DeviceList: React.FC<DeviceListProps> = ({ firebaseDevices }) => {
  return (
    <Row justify="start" style={{ width: '100%' }}>
      {firebaseDevices
        .sort((deviceA, deviceB) => (deviceA.status ? deviceA.uid.localeCompare(deviceB.uid) : 1))
        .map((device) => {
          // if (!device.status)
          //   return (
          //     <Col key={device.uid}>
          //       <DeviceCardSkeleton
          //         key={device.uid}
          //         name={device.type}
          //         description="Device is offline"
          //       />
          //     </Col>
          //   );

          return <DeviceCard firebaseDevice={device} key={device.uid} />;
        })}
    </Row>
  );
};

export default DeviceList;
