import { updateDeviceData } from '@/services/firebase';
import { sendRequest } from '@/services/mqtt';
import { getRandomShortUid } from '@/utils/utils';
import { Watermixer, Device } from '@gbaranski/types';
import { MqttClient } from 'mqtt';

const MILLIS_IN_SECOND = 1000;
const SECOND_IN_MINUTE = 60;
const MIX_MINUTES = 10;

export default () => {
  const mixWater = (device: Device.FirebaseDevice, mqttClient: MqttClient) => {
    const request: Device.Request = {
      correlationData: getRandomShortUid(),
    };

    const onSuccess = () => {
      const finishMixTimestamp = Date.now() + MILLIS_IN_SECOND * SECOND_IN_MINUTE * MIX_MINUTES;
      updateDeviceData(device.uid, {
        finishMixTimestamp,
      });
    };

    sendRequest({
      request,
      topic: Watermixer.getStartMixingTopic(device.uid),
      mqttClient,
      onSuccess,
    });
  };

  return {
    mixWater,
  };
};
