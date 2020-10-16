import React, { useEffect } from 'react';
import { Table, Popconfirm } from 'antd';
import { deleteDevice } from '@/services/firebase';
import { Device } from '@houseflow/types';
import { useModel } from 'umi';

export default () => {
  const { allFirebaseDevices, getAndSetAllDevices } = useModel('deviceData');
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
          onConfirm={() => {
            deleteDevice(record).then(() => {
              getAndSetAllDevices();
            });
          }}
          okText="Yes"
          cancelText="No"
        >
          <a href="#">Delete</a>
        </Popconfirm>
      ),
    },
  ];

  useEffect(() => {
    getAndSetAllDevices();
  }, []);
  return (
    <>
      <Table
        columns={columns}
        dataSource={allFirebaseDevices.map((device, index) => {
          return {
            key: index,
            type: device.type,
            uid: device.uid,
          };
        })}
      />
    </>
  );
};
