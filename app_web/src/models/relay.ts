import { updateDeviceData } from '@/services/firebase';
import { sendRequest } from '@/services/mqtt';
import { getRandomShortUid } from '@/utils/utils';
import { Relay, Device } from '@houseflow/types';
import { message } from 'antd';
import { MqttClient } from 'mqtt';

export type TimestampFunc = () => number;

export default () => {
  const sendRelaySignal = async (
    device: Device.FirebaseDevice,
    mqttClient: MqttClient,
    getTimestampFunc: TimestampFunc,
  ) => {
    const request: Device.Request = {
      correlationData: getRandomShortUid(),
    };

    try {
      message.info('Sending!');
      await sendRequest({
        request,
        topic: Relay.getSendSignalTopic(device.uid),
        mqttClient,
      });

      updateDeviceData(device.uid, {
        lastSignalTimestamp: getTimestampFunc(),
      });
    } catch (e) {
      console.log(`Error when sending request ${e}`);
      message.error(e.message);
    }
  };

  return {
    sendRelaySignal,
  };
};
