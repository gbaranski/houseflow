import React from 'react';
import { Card, Statistic, Tooltip } from 'antd';
import { CarOutlined } from '@ant-design/icons';
import { Device, Relay } from '@houseflow/types';
import { useModel } from 'umi';
import moment from 'moment';
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
          <CarOutlined key="OpenGate" onClick={() => openGate(device, mqtt)} />
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
