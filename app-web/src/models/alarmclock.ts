import { Client, DateTime, State } from '@gbaranski/types';
import { getWebsocket } from '@/services/websocket';
import { useState } from 'react';
import moment, { Moment } from 'moment';

export default () => {
  const [timeModalVisible, setTimeModalVisible] = useState<boolean>(false);
  const [newAlarmTime, setNewAlarmTime] = useState<Moment | null>(moment(new Date(), 'HH:mm'));

  const testSiren = (uid: string) => {
    const req: Client.Request = {
      deviceUid: uid,
      requestType: 'TEST_SIREN',
    };
    getWebsocket()?.send(JSON.stringify(req));
  };

  const setState = (uid: string, newState: State) => {
    const req: Client.Request = {
      deviceUid: uid,
      requestType: 'SET_STATE',
      data: { state: newState },
    };
    getWebsocket()?.send(JSON.stringify(req));
  };

  const sendNewAlarmTime = (dateTime: DateTime, uid: string) => {
    if (!newAlarmTime) throw new Error('Alarm time state is not defined');

    const req: Client.Request = {
      requestType: 'SET_TIME',
      deviceUid: uid,
      data: dateTime,
    };
    console.log(req);
    getWebsocket()?.send(JSON.stringify(req));
  };

  return {
    timeModalVisible,
    setTimeModalVisible,
    newAlarmTime,
    setNewAlarmTime,
    testSiren,
    sendNewAlarmTime,
    setState,
  };
};
