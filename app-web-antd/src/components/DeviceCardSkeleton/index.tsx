import React from 'react';
import { Card, Empty } from 'antd';
import { Link } from 'umi';
import { EnterOutlined } from '@ant-design/icons';

export default (props: { name: string }) => {
  return (
    <Card
      title={props.name}
      style={{ width: 300 }}
      actions={[
        <Link to="#">
          <EnterOutlined key="Enter" />
        </Link>,
      ]}
    >
      <Empty />
    </Card>
  );
};
