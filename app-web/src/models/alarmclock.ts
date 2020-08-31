import { DateTime, State } from '@gbaranski/types';
import { useState } from 'react';
import moment, { Moment } from 'moment';

export default () => {
  const [timeModalVisible, setTimeModalVisible] = useState<boolean>(false);
  const [newAlarmTime, setNewAlarmTime] = useState<Moment | null>(moment(new Date(), 'HH:mm'));

  const testSiren = (uid: string) => {
    console.log("Not implemented")
    // const req: Device.RequestDevice = {
    //   topic: {
    //     name: 'testsiren',
    //     uid: uid,
    //   }
    // };
    // getWebsocket()?.send(JSON.stringify(req));
  };

  const setState = (uid: string, newState: State) => {
    console.log("Not implemented")
    // const req: Device.RequestDevice = {
    //   topic: {
    //     name: 'setstate',
    //     uid,
    //   },
    //   data: newState,
    // };
    // getWebsocket()?.send(JSON.stringify(req));
  };

  const sendNewAlarmTime = (dateTime: DateTime, uid: string) => {
    console.log("Not implemented")
    // if (!newAlarmTime) throw new Error('Alarm time state is not defined');

    // const req: Device.RequestDevice = {
    //   topic: {
    //     name: 'settime',
    //     uid,
    //   },
    //   data: dateTime,
    // };
    // console.log(req);
    // getWebsocket()?.send(JSON.stringify(req));
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
