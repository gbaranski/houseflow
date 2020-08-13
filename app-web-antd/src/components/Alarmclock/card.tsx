import React from 'react';
import { Card, Statistic, Row, Col } from 'antd';
import { ArrowUpOutlined, EnterOutlined } from '@ant-design/icons';
import { Link } from 'umi';

export default () => {
  return (
    <Card
      title="Alarmclock"
      style={{ width: 300 }}
      actions={[
        <Link to="#">
          <EnterOutlined key="Enter" />
        </Link>,
      ]}
    >
      <Row>
        <Col span={12}>
          <Statistic
            title="Temperature"
            value={25.4}
            precision={1}
            valueStyle={{ color: '#3f8600' }}
            prefix={<ArrowUpOutlined />}
            suffix="°"
          />
        </Col>

        <Col span={12}>
          <Statistic
            title="Humidity"
            value={56}
            precision={1}
            valueStyle={{ color: '#3f8600' }}
            prefix={<ArrowUpOutlined />}
            suffix="%"
          />
        </Col>
      </Row>
      <Row>
        <Col span={12}>
          <Statistic title="Alarm time" value="12:30" />
        </Col>
        <Col span={12}>
          <Statistic title="Remaining time" value="10:49" />
        </Col>
      </Row>
    </Card>
  );
};
