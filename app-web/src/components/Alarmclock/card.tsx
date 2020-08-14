import React from 'react';
import { Card, Statistic, Row, Col, Tooltip } from 'antd';
import { ClockCircleOutlined, PoweroffOutlined, WarningOutlined } from '@ant-design/icons';
import { Device, Alarmclock } from '@gbaranski/types';
import { parseDateTime } from '@/utils/utils';
import Icon from '@mdi/react';
import { mdiThermometer, mdiWaterPercent } from '@mdi/js';

interface AlarmclockCardProps {
  device: Device.ActiveDevice<Alarmclock.Data>;
}

const AlarmclockCard: React.FC<AlarmclockCardProps> = ({ device }) => {
  return (
    <Card
      title="Alarmclock"
      style={{ width: 300 }}
      bodyStyle={{ minHeight: 180 }}
      actions={[
        <Tooltip title="Set time">
          <ClockCircleOutlined key="setTime" />
        </Tooltip>,
        <Tooltip title="Test alarm">
          <WarningOutlined key="testAlarm" />
        </Tooltip>,
        <Tooltip title="Switch state">
          <PoweroffOutlined key="switchState" />
        </Tooltip>,
      ]}
    >
      <Row>
        <Col span={12}>
          <Statistic
            title="Temperature"
            value={device.data.sensor.temperature}
            precision={1}
            valueStyle={{ position: 'relative', right: 12, bottom: 3 }}
            prefix={
              <Icon
                path={mdiThermometer}
                size={1.1}
                style={{ position: 'relative', top: 4, left: 5 }}
              />
            }
            suffix="°"
          />
        </Col>

        <Col span={12}>
          <Statistic
            title="Humidity"
            value={device.data.sensor.humidity}
            precision={1}
            valueStyle={{ position: 'relative', right: 12, bottom: 4 }}
            prefix={
              <Icon
                path={mdiWaterPercent}
                size={1.2}
                style={{ position: 'relative', top: 5, left: 5 }}
              />
            }
            suffix="%"
          />
        </Col>
      </Row>
      <Row>
        <Col span={12}>
          <Statistic title="Alarm time" value={parseDateTime(device.data.alarmTime)} />
        </Col>
        <Col span={12}>
          <Statistic title="Remaining time" value="10:49" />
        </Col>
      </Row>
    </Card>
  );
};

export default AlarmclockCard;
