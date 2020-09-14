import React, { useEffect } from 'react';
import { PageContainer } from '@ant-design/pro-layout';
import { Card, Row } from 'antd';
import { useModel } from 'umi';
import DeviceList from '@/components/DevicesList';

export default (): React.ReactNode => {
  const {
    getAndSetFirebaseDevices,
    firebaseDevices,
    activeDevices,
    getActiveDevices,
    setDataListeners,
  } = useModel('deviceData');
  const { initialState } = useModel('@@initialState');
  const { firebaseUser, socket } = initialState || {};

  useEffect(() => {
    getAndSetFirebaseDevices(firebaseUser);
    if (!socket) throw new Error('Socket is not defined');
    getActiveDevices(socket);
    setDataListeners(socket);
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
