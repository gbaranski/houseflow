import React from 'react';
import NewDevice from '@/components/NewDevice';
import { PageContainer } from '@ant-design/pro-layout';
import { Card } from 'antd';
import AdminDeviceTable from '@/components/AdminDeviceTable';

export default () => {
  return (
    <PageContainer>
      <Card>
        <NewDevice />
        <AdminDeviceTable />
      </Card>
    </PageContainer>
  );
};
