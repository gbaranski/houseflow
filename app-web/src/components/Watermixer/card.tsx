import React from 'react';
import { Card, Statistic, Row, Col, Tooltip } from 'antd';
import { CoffeeOutlined } from '@ant-design/icons';
import { useModel } from 'umi';
import { Device, Watermixer } from '@gbaranski/types';
import { PageLoading } from '@ant-design/pro-layout';

interface WatermixerCardProps {
  device: Device.ActiveDevice<Watermixer.Data>;
}
const WatermixerCard: React.FC<WatermixerCardProps> = ({ device }) => {
  const { initialState } = useModel('@@initialState');
  const { socket } = initialState || {};
  const { mixWater } = useModel('watermixer');

  const hasElapsed = (timestamp: number) => Date.now() > timestamp;

  if (!socket) return <PageLoading />;

  return (
    <Card
      title="Watermixer"
      style={{ width: 300 }}
      bodyStyle={{ minHeight: 180 }}
      actions={[
        <Tooltip title="Start mixing">
          <CoffeeOutlined key="Mix" onClick={() => mixWater(device.uid, socket)} />
        </Tooltip>,
      ]}
    >
      <Row justify="space-around">
        <Col span={10} style={{ textAlign: 'left' }}>
          <Statistic
            title="Mixing state"
            value={hasElapsed(device.data.finishMixTimestamp) ? 'Idle' : 'Mixing!'}
          />
        </Col>
        <Col span={10} style={{ textAlign: 'right' }}>
          <Statistic.Countdown
            title="Time left"
            value={device.data.finishMixTimestamp}
            format="mm:ss"
          />
        </Col>
      </Row>
    </Card>
  );
};

export default WatermixerCard;
