import React from 'react';
import { Card, Statistic } from 'antd';
import { Device, Relay } from '@houseflow/types';
import { useModel } from 'umi';
import moment from 'moment';
import { mdiGarageVariant } from '@mdi/js';
import { CARD_MIN_HEIGHT, CARD_MIN_WIDTH } from '@/utils/constants';
import PageLoading from '@/components/PageLoading';
import DeviceAction from '@/components/DeviceAction';

interface GarageCardProps {
  device: Device.FirebaseDevice<Relay.Data>;
}
const GarageCard: React.FC<GarageCardProps> = ({ device }) => {
  const { initialState } = useModel('@@initialState');
  const { mqtt } = initialState || {};

  const { sendRelaySignal } = useModel('relay');

  if (!mqtt) return <PageLoading />;

  const openGarage = () => sendRelaySignal(device, mqtt, () => Date.now());

  return (
    <Card
      title="Garage"
      style={{ minWidth: CARD_MIN_WIDTH }}
      bodyStyle={{ minHeight: CARD_MIN_HEIGHT }}
      actions={[
        <DeviceAction
          mdiIconPath={mdiGarageVariant}
          toolTipTitle="Open garage"
          onSubmit={openGarage}
        />,
      ]}
    >
      <Statistic
        title="Last open"
        value={device.data.lastSignalTimestamp}
        formatter={(value) => moment(value).fromNow()}
      />
    </Card>
  );
};

export default GarageCard;
