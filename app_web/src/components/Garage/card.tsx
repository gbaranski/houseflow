import React from 'react';
import { Card, Statistic, Tooltip } from 'antd';
import { Device, Relay } from '@houseflow/types';
import { useModel } from 'umi';
import moment from 'moment';
import Icon from '@mdi/react';
import { mdiGarageVariant } from '@mdi/js';
import PageLoading from '../PageLoading';

interface GarageCardProps {
  device: Device.FirebaseDevice<Relay.Data>;
}
const GarageCard: React.FC<GarageCardProps> = ({ device }) => {
  const { initialState } = useModel('@@initialState');
  const { mqtt } = initialState || {};

  const { sendRelaySignal } = useModel('relay');

  if (!mqtt) return <PageLoading />;

  return (
    <Card
      title="Gate"
      style={{ width: 300 }}
      bodyStyle={{ minHeight: 180 }}
      actions={[
        <Tooltip title="Open garage">
          <a onClick={() => sendRelaySignal(device, mqtt, () => Date.now())}>
            <Icon path={mdiGarageVariant} size={1} />
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

export default GarageCard;
