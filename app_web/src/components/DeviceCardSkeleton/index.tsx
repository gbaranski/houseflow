import React from 'react';
import { Card, Empty } from 'antd';
import { Link } from 'umi';
import { EnterOutlined } from '@ant-design/icons';
import { capitalizeFirst } from '@/utils/utils';

export default (props: { name: string, description: string }) => {
  return (
    <Card
      title={capitalizeFirst(props.name.toLowerCase())}
      style={{ width: 300 }}
      bodyStyle={{ minHeight: 180 }}
      actions={[
        <Link to="#">
          <EnterOutlined key="Enter" />
        </Link>,
      ]}
    >
      <Empty description={props.description} />
    </Card>
  );
};
