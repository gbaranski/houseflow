import React, { useEffect } from 'react';
import { PageContainer } from '@ant-design/pro-layout';
import { Card, Row } from 'antd';
import { useModel } from 'umi';
import DeviceList from '@/components/DevicesList';

export default (): React.ReactNode => {
  const { setupListeners, getAndSetFirebaseDevices, firebaseDevices, activeDevices } = useModel(
    'deviceData',
  );
  const { initialState } = useModel('@@initialState');
  const { firebaseUser } = initialState || {};
  setupListeners();
  useEffect(() => {
    getAndSetFirebaseDevices(firebaseUser);
  }, []);

  return (
    <PageContainer>
      <Card>
        <Row>
          <DeviceList activeDevices={activeDevices} firebaseDevices={firebaseDevices} />
        </Row>
      </Card>
    </PageContainer>
  );
};
