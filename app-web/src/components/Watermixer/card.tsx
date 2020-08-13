import React from 'react';
import { Card, Statistic, Row, Col } from 'antd';
import { EnterOutlined, CoffeeOutlined } from '@ant-design/icons';
import { useModel } from 'umi';
import { Device, Watermixer } from '@gbaranski/types';
import { parseSeconds, parseWaterBoolean } from '@/utils/utils';

interface WatermixerCardProps {
  device: Device.ActiveDevice<Watermixer.Data>;
}
const WatermixerCard: React.FC<WatermixerCardProps> = ({ device }) => {
  const { mixWater } = useModel('watermixer');

  return (
    <Card
      title="Watermixer"
      style={{ width: 300 }}
      bodyStyle={{ minHeight: 180 }}
      actions={[
        <EnterOutlined key="Enter" />,
        <CoffeeOutlined key="Mix" onClick={() => mixWater(device.uid)} />,
      ]}
    >
      <Row justify="space-around">
        <Col span={10} style={{ textAlign: 'left' }}>
          <Statistic title="Mixing state" value={parseWaterBoolean(device.data.isTimerOn)} />
        </Col>
        <Col span={10} style={{ textAlign: 'right' }}>
          <Statistic title="Time left" value={parseSeconds(device.data.remainingSeconds)} />
        </Col>
      </Row>
    </Card>
  );
};

export default WatermixerCard;
