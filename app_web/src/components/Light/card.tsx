import React from 'react';
import { Card } from 'antd';
import { Device, Light } from '@houseflow/types';
import { useModel } from 'umi';
import { CARD_MIN_HEIGHT, CARD_MIN_WIDTH } from '@/utils/constants';
import { mdiLamp } from '@mdi/js';
import { sendDeviceRequest } from '@/services/device';
import DeviceAction from '../DeviceAction';

interface LightCardProps {
  device: Device.FirebaseDevice<Light.Data>;
}
const LightCard: React.FC<LightCardProps> = ({ device }) => {
  const { initialState } = useModel('@@initialState');
  const { currentUser } = initialState || {};
  if (!currentUser) throw new Error('Current user is not defined');

  const toggleLights = async () =>
    sendDeviceRequest({
      user: {
        token: await currentUser.getIdToken(),
      },
      device: Light.createLightRequest({ uid: device.uid }),
    });

  return (
    <Card
      title="Lights"
      style={{ minWidth: CARD_MIN_WIDTH }}
      bodyStyle={{ minHeight: CARD_MIN_HEIGHT }}
      actions={[
        <DeviceAction mdiIconPath={mdiLamp} toolTipTitle="Toggle lights" onSubmit={toggleLights} />,
      ]}
    >
      {`State: ${device.data.currentState}`}
    </Card>
  );
};

export default LightCard;
