import React, { useEffect } from 'react';
import { PageContainer } from '@ant-design/pro-layout';
import { Card, Table } from 'antd';
import { useModel } from 'umi';
import { sendCurrentConnectionsRequest } from '@/services/websocket';
import { Device, Client } from '@gbaranski/types';

const DeviceTable = (props: {
  online: Device.ActiveDevice[];
  offline: Device.FirebaseDevice[];
}) => {
  const columns = [
    {
      title: 'UID',
      dataIndex: 'uid',
      key: 'uid',
    },
    {
      title: 'Type',
      dataIndex: 'type',
      key: 'type',
    },

    {
      title: 'IP',
      dataIndex: 'ip',
      key: 'ip',
    },
  ];

  const data = props.offline.map((device, index) => {
    return {
      key: index,
      uid: device.uid,
      type: device.type,
      ip: props.online.find((_device) => _device.uid === device.uid)?.ip || 'Offline',
    };
  });
  return <Table dataSource={data} columns={columns} />;
};

const ClientTable = (props: { clients: Client.ActiveUser[] }) => {
  const columns = [
    {
      title: 'UID',
      dataIndex: 'uid',
      key: 'uid',
    },
    {
      title: 'Role',
      dataIndex: 'role',
      key: 'role',
    },
    {
      title: 'IP',
      dataIndex: 'ip',
      key: 'ip',
    },
  ];
  const data = props.clients.map((client, index) => {
    return {
      key: index,
      ...client,
    };
  });
  return <Table dataSource={data} columns={columns} />;
};

export default () => {
  const {
    allConnections,
    allFirebaseDevices,
    getAndSetAllDevices,
    setupListeners,
    getAndSetFirebaseDevices,
  } = useModel('deviceData');
  const { initialState } = useModel('@@initialState');
  const { firebaseUser } = initialState || {};
  setupListeners();

  useEffect(() => {
    getAndSetFirebaseDevices(firebaseUser).then(() => sendCurrentConnectionsRequest());
    if (allFirebaseDevices.length > 1) return;
    getAndSetAllDevices();
  }, []);

  return (
    <PageContainer>
      <Card>
        <DeviceTable online={allConnections?.devices || []} offline={allFirebaseDevices} />
        <ClientTable clients={allConnections?.clients || []} />
      </Card>
    </PageContainer>
  );
};
