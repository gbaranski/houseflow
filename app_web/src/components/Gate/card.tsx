import React from 'react';
import { Card, Statistic } from 'antd';
import { Device, Relay } from '@houseflow/types';
import { useModel } from 'umi';
import moment from 'moment';
import { CARD_MIN_HEIGHT, CARD_MIN_WIDTH } from '@/utils/constants';
import { mdiGate } from '@mdi/js';
import PageLoading from '../PageLoading';
import DeviceAction from '../DeviceAction';

interface GateCardProps {
  device: Device.FirebaseDevice<Relay.Data>;
}
const GateCard: React.FC<GateCardProps> = ({ device }) => {
  const { initialState } = useModel('@@initialState');
  const { mqtt } = initialState || {};

  const { sendRelaySignal } = useModel('relay');

  if (!mqtt) return <PageLoading />;

  const openGate = () => sendRelaySignal(device, mqtt, () => Date.now());

  return (
    <Card
      title="Gate"
      style={{ minWidth: CARD_MIN_WIDTH }}
      bodyStyle={{ minHeight: CARD_MIN_HEIGHT }}
      actions={[
        <DeviceAction mdiIconPath={mdiGate} toolTipTitle="Open gate" onSubmit={openGate} />,
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

export default GateCard;
