import { updateDeviceData } from '@/services/firebase';
import { sendRequest } from '@/services/mqtt';
import { getRandomShortUid } from '@/utils/utils';
import { Watermixer, Device } from '@houseflow/types';
import { message } from 'antd';
import { MqttClient } from 'mqtt';

const MILLIS_IN_SECOND = 1000;
const SECOND_IN_MINUTE = 60;
const MIX_MINUTES = 10;

export default () => {
  const mixWater = async (device: Device.FirebaseDevice, mqttClient: MqttClient) => {
    const request: Device.Request = {
      correlationData: getRandomShortUid(),
    };

    try {
      message.info('Sending!');
      await sendRequest({
        request,
        topic: Watermixer.getStartMixingTopic(device.uid),
        mqttClient,
      });
      const finishMixTimestamp = Date.now() + MILLIS_IN_SECOND * SECOND_IN_MINUTE * MIX_MINUTES;
      updateDeviceData(device.uid, {
        finishMixTimestamp,
      });
    } catch (e) {
      console.log(`Error when sending request ${e}`);
      message.error(e.message);
    }
  };

  return {
    mixWater,
  };
};
