import React, { useState } from 'react';

import { Device, Client, Light, Relay } from '@houseflow/types';
import { Card, Col, Popconfirm, Typography } from 'antd';
import { mdiGarage, mdiGate, mdiHotTub, mdiLamp } from '@mdi/js';
import Icon from '@mdi/react';
import { capitalizeFirst } from '@/utils/utils';
import { sendDeviceRequest } from '@/services/device';
import { useModel } from 'umi';

interface Action {
  createRequest: ({ uid }: { uid: string }) => Client.DeviceRequest['device'];
  title: string;
}

interface UIDevice {
  type: Device.DeviceType;
  icon: string;
  color: string;
  action: Action;
}

const uiDevices: UIDevice[] = [
  {
    type: 'LIGHT',
    color: '#ffa000',
    icon: mdiLamp,
    action: {
      title: 'toggle lights',
      createRequest: Light.createLightRequest,
    },
  },
  {
    type: 'WATERMIXER',
    color: 'rgb(79, 119, 149)',
    icon: mdiHotTub,
    action: {
      title: 'mix water',
      createRequest: Relay.createRelayRequest,
    },
  },
  {
    type: 'GATE',
    color: 'rgb(103, 151, 109)',
    icon: mdiGate,
    action: {
      title: 'open gate',
      createRequest: Relay.createRelayRequest,
    },
  },
  {
    type: 'GARAGE',
    color: 'rgb(183, 111, 110)',
    icon: mdiGarage,
    action: {
      title: 'open garage',
      createRequest: Relay.createRelayRequest,
    },
  },
];

interface DeviceCardProps {
  firebaseDevice: Device.FirebaseDevice;
}

const DeviceCard = ({ firebaseDevice }: DeviceCardProps) => {
  const [popconfirmVisible, setPopconfirmVisible] = useState(false);
  const [loading, setLoading] = useState(false);

  const { initialState } = useModel('@@initialState');
  const { currentUser } = initialState || {};

  const uiDevice: UIDevice | undefined = uiDevices.find(
    (device) => device.type === firebaseDevice.type,
  );
  if (!uiDevice)
    return (
      <Card>
        <Typography>Unrecognized device</Typography>
      </Card>
    );

  const onSubmit = async () => {
    if (!currentUser) throw new Error("Couldn't retreive current user");
    console.log('Sending!');
    await sendDeviceRequest({
      user: {
        token: await currentUser.getIdToken(),
      },
      device: uiDevice.action.createRequest({ uid: firebaseDevice.uid }),
    });
  };

  return (
    <Col>
      <Card
        bodyStyle={{ backgroundColor: uiDevice.color, width: '100%' }}
        hoverable
        bordered={false}
        onClick={() => setPopconfirmVisible(true)}
      >
        <Col style={{ textAlign: 'center' }}>
          <Icon path={uiDevice.icon} size={5} color="rgba(255,255,255, 0.7)" />
          <Typography style={{ fontSize: 24, color: 'rgba(255,255,255, 0.8)' }}>
            {capitalizeFirst(uiDevice.type.toLowerCase())}
          </Typography>
        </Col>
      </Card>
      <Popconfirm
        title={`Are you sure to ${uiDevice.action.title} now?`}
        visible={popconfirmVisible}
        okButtonProps={{ loading }}
        placement="bottom"
        onCancel={() => {
          setPopconfirmVisible(false);
        }}
        onConfirm={() => {
          setLoading(true);

          onSubmit().finally(() => {
            setLoading(false);
            setPopconfirmVisible(false);
          });
        }}
      />
    </Col>
  );
};

export default DeviceCard;
