import React from 'react';
import { Card, Statistic, Tooltip } from 'antd';
import { Device, Relay } from '@houseflow/types';
import { useModel } from 'umi';
import moment from 'moment';
import Icon from '@mdi/react';
import { mdiGate } from '@mdi/js';
import { CARD_MIN_HEIGHT, CARD_MIN_WIDTH } from '@/utils/constants';
import PageLoading from '../PageLoading';

interface GateCardProps {
  device: Device.FirebaseDevice<Relay.Data>;
}
const GateCard: React.FC<GateCardProps> = ({ device }) => {
  const { initialState } = useModel('@@initialState');
  const { mqtt } = initialState || {};

  const { sendRelaySignal } = useModel('relay');

  if (!mqtt) return <PageLoading />;

  return (
    <Card
      title="Gate"
      style={{ minWidth: CARD_MIN_WIDTH }}
      bodyStyle={{ minHeight: CARD_MIN_HEIGHT }}
      actions={[
        <Tooltip title="Open gate">
          <a onClick={() => sendRelaySignal(device, mqtt, () => Date.now())}>
            <Icon path={mdiGate} size={1} />
          </a>
        </Tooltip>,
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
