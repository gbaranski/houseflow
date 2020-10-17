import React from 'react';
import { Card, Statistic, Tooltip } from 'antd';
import { Device, Relay } from '@houseflow/types';
import { useModel } from 'umi';
import moment from 'moment';
import Icon from '@mdi/react';
import { mdiGate } from '@mdi/js';
import PageLoading from '../PageLoading';

interface GateCardProps {
  device: Device.FirebaseDevice<Relay.Data>;
}
const GateCard: React.FC<GateCardProps> = ({ device }) => {
  const { initialState } = useModel('@@initialState');
  const { mqtt } = initialState || {};

  const { openGate } = useModel('gate');

  if (!mqtt) return <PageLoading />;

  return (
    <Card
      title="Gate"
      style={{ width: 300 }}
      bodyStyle={{ minHeight: 180 }}
      actions={[
        <Tooltip title="Open gate">
          <a onClick={() => openGate(device, mqtt)}>
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
