import React, { useEffect } from 'react';
import { PageContainer, PageLoading } from '@ant-design/pro-layout';
import { Card, Empty, Row } from 'antd';
import { useModel } from 'umi';
import DeviceList from '@/components/DevicesList';

export default (): React.ReactNode => {
  const { firebaseDevices, initializeFirebaseDevices } = useModel('device');
  const { initialState } = useModel('@@initialState');
  const { firebaseUser } = initialState || {};
  if (!firebaseUser) return <PageLoading />;

  useEffect(() => {
    initializeFirebaseDevices(firebaseUser);
  }, []);

  return (
    <PageContainer>
      <Card>
        {firebaseDevices.length > 0 ? (
          <Row>
            <DeviceList firebaseDevices={firebaseDevices} />
          </Row>
        ) : (
          <Row justify="center">
            <Empty
              style={{ color: 'rgba(0,0,0,.50)' }}
              // TODO: Add some link for contact
              description="No devices, if you think thats an issue, contact us"
            />
          </Row>
        )}
      </Card>
    </PageContainer>
  );
};
