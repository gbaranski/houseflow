import React, { useEffect } from 'react';
import { PageContainer, PageLoading } from '@ant-design/pro-layout';
import { Card, Row } from 'antd';
import { useModel } from 'umi';
import DeviceList from '@/components/DevicesList';

export default (): React.ReactNode => {
  const { firebaseDevices, initializeFirebaseDevices } = useModel('deviceData');
  const { initialState } = useModel('@@initialState');
  const { firebaseUser } = initialState || {};
  if (!firebaseUser) return <PageLoading />;

  useEffect(() => {
    initializeFirebaseDevices(firebaseUser);
  }, []);

  return (
    <PageContainer>
      <Card>
        <Row>
          <DeviceList firebaseDevices={firebaseDevices} />
        </Row>
      </Card>
    </PageContainer>
  );
};
