import React from 'react';
import NewDevice from '@/components/NewDevice';
import { PageContainer } from '@ant-design/pro-layout';
import { Card } from 'antd';

export default () => {
  return (
    <PageContainer>
      <Card>
        <NewDevice />
      </Card>
    </PageContainer>
  );
};
