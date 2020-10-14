import React from 'react';
import { Card, Statistic, Tooltip } from 'antd';
import { CarOutlined } from '@ant-design/icons';
import { Device, Gate } from '@gbaranski/types';
import { useModel } from 'umi';
import PageLoading from '../PageLoading';
import moment from 'moment';

interface GateCardProps {
  device: Device.FirebaseDevice<Gate.Data>;
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
        value={device.data.lastOpenTimestamp}
        formatter={(value, config) => moment(value).fromNow()}
      />
    </Card>
  );
};

export default GateCard;
