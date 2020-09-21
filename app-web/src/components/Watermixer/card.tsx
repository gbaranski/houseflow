import React from 'react';
import { Card, Statistic, Row, Col, Tooltip } from 'antd';
import { CoffeeOutlined } from '@ant-design/icons';
import { Device, Watermixer } from '@gbaranski/types';

interface WatermixerCardProps {
  device: Device.FirebaseDevice<Watermixer.Data>;
}
const WatermixerCard: React.FC<WatermixerCardProps> = ({ device }) => {
  const hasElapsed = (timestamp: number) => Date.now() > timestamp;

  return (
    <Card
      title="Watermixer"
      style={{ width: 300 }}
      bodyStyle={{ minHeight: 180 }}
      actions={[
        <Tooltip title="Start mixing">
          <CoffeeOutlined
            key="Mix"
            onClick={() => {
              throw new Error('Not implemented');
            }}
          />
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
