import React, { useEffect } from 'react';
import { Table, Popconfirm } from 'antd';
import { deleteDevice } from '@/services/firebase';
import { Device } from '@gbaranski/types';
import { useModel } from 'umi';

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
  const { allDevices, getAndSetAllDevices } = useModel('deviceData');

  useEffect(() => {
    if (allDevices.length > 1) return;
    getAndSetAllDevices();
  }, []);
  return (
    <Table
      columns={columns}
      dataSource={allDevices.map((device, index) => {
        return {
          key: index,
          type: device.type,
          uid: device.uid,
        };
      })}
    />
  );
};
