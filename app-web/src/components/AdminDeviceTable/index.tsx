import React, { useEffect, useState } from 'react';
import { Table, Popconfirm } from 'antd';
import { getAllDevices, deleteDevice } from '@/services/firebase';
import { Device } from '@gbaranski/types';

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
    title: 'Action',
    key: 'action',
    render: (record: Device.FirebaseDevice) => (
      <Popconfirm
        title="Are you sure delete this task?"
        onConfirm={() => deleteDevice(record)}
        okText="Yes"
        cancelText="No"
      >
        <a href="#">Delete</a>
      </Popconfirm>
    ),
  },
];

export default () => {
  const [dataSource, setDataSource] = useState<any[]>([]);

  const getAndSetDataSource = () => {
    getAllDevices().then((firebaseDevice) => {
      setDataSource(
        firebaseDevice.map((device, index) => {
          return {
            key: index,
            type: device.type,
            uid: device.uid,
          };
        }),
      );
    });
  };

  useEffect(() => {
    getAndSetDataSource();
  });
  return <Table columns={columns} dataSource={dataSource} />;
};
