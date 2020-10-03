import React, { useState } from 'react';
import { Card, Statistic, Row, Col, Tooltip, Popconfirm, Modal, TimePicker } from 'antd';
import { ClockCircleOutlined, PoweroffOutlined, WarningOutlined } from '@ant-design/icons';
import { Device, Alarmclock, DateTime, Client } from '@gbaranski/types';
import { parseDateTime, getOnFinishTime } from '@/utils/utils';
import Icon from '@mdi/react';
import { mdiThermometer, mdiWaterPercent } from '@mdi/js';
import { useModel } from 'umi';

interface AlarmclockCardProps {
  device: Device.ActiveDevice<Alarmclock.Data>;
}

const AlarmclockCard: React.FC<AlarmclockCardProps> = ({ device }) => {
  const {
    testSiren,
    timeModalVisible,
    setTimeModalVisible,
    sendNewAlarmTime,
    newAlarmTime,
    setNewAlarmTime,
  } = useModel('alarmclock');

  const [modalLoading, setModalLoading] = useState(false);

  const onModalOk = () => {
    if (!newAlarmTime) throw new Error('Alarm time not defined');
    const dateTime: DateTime = {
      hour: newAlarmTime.hour(),
      minute: newAlarmTime.minute(),
      second: 0,
    };
    setModalLoading(true);
    sendNewAlarmTime(dateTime, device.uid);

    const msgCallback = (message: MessageEvent) => {
      const response = JSON.parse(message.data) as Client.Response;
      if (!response.data || response.requestType !== 'DATA') return;
      const parsedMsg = response.data as Device.ActiveDevice[];
      const targetDevice = parsedMsg.find((_device) => _device.uid === device.uid);
      if (!targetDevice) throw new Error('Could not find target device');
      const alarmData = (targetDevice.data as Alarmclock.Data).alarmTime;

      if (alarmData.hour === dateTime.hour && alarmData.minute === dateTime.minute) {
        console.log('Cleaning listener');
        setModalLoading(false);
        setTimeModalVisible(false);
        getWebsocket()?.removeEventListener('message', msgCallback);
      }
    };

    getWebsocket()?.addEventListener('message', msgCallback);
  };

  return (
    <Card
      title="Alarmclock"
      style={{ width: 300 }}
      bodyStyle={{ minHeight: 180 }}
      actions={[
        <Tooltip title="Set time">
          <ClockCircleOutlined key="setTime" onClick={() => setTimeModalVisible(true)} />
        </Tooltip>,
        <Popconfirm
          title="Are you sure you want to test siren? It might be so loud"
          onConfirm={() => testSiren(device.uid)}
          okText="Yes"
          cancelText="No"
        >
          <Tooltip title="Test alarm">
            <WarningOutlined key="testAlarm" />
          </Tooltip>
        </Popconfirm>,
        <Tooltip title="Switch state">
          <PoweroffOutlined
            key="switchState"
            // onClick={() => setState(device.uid, !device.data.alarmState)}
          />
        </Tooltip>,
      ]}
    >
      <Modal
        title="Basic Modal"
        visible={timeModalVisible}
        onOk={onModalOk}
        onCancel={() => setTimeModalVisible(false)}
        confirmLoading={modalLoading}
      >
        <p>Set new alarm time, modal will be automaticlly closed.</p>
        <TimePicker format="HH:mm" value={newAlarmTime} onChange={(e) => setNewAlarmTime(e)} />
      </Modal>
      <Row>
        <Col span={12}>
          <Statistic
            title="Temperature"
            value={device.data.sensor.temperature}
            precision={1}
            valueStyle={{ position: 'relative', right: 12, bottom: 3 }}
            prefix={
              <Icon
                path={mdiThermometer}
                size={1.1}
                style={{ position: 'relative', top: 4, left: 5 }}
              />
            }
            suffix="°"
          />
        </Col>

        <Col span={12}>
          <Statistic
            title="Humidity"
            value={device.data.sensor.humidity}
            precision={1}
            valueStyle={{ position: 'relative', right: 12, bottom: 4 }}
            prefix={
              <Icon
                path={mdiWaterPercent}
                size={1.2}
                style={{ position: 'relative', top: 5, left: 5 }}
              />
            }
            suffix="%"
          />
        </Col>
      </Row>
      <Row>
        <Col span={12}>
          <Statistic
            title="Alarm time"
            value={parseDateTime(device.data.alarmTime)}
            // suffix={parseAlarmclockBoolean(device.data.alarmState)}
          />
        </Col>
        <Col span={12}>
          <Statistic.Countdown
            title="Remaining time"
            // @ts-ignore
            value={getOnFinishTime(device.data.alarmTime)}
          />
        </Col>
      </Row>
    </Card>
  );
};

export default AlarmclockCard;
