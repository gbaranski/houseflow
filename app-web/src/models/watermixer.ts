import { updateDevice } from '@/services/firebase';
import { getRandomShortUid } from '@/utils/utils';
import { Watermixer, Device } from '@gbaranski/types';
import { MqttClient } from 'mqtt';

const MILLIS_IN_SECOND = 1000;
const SECOND_IN_MINUTE = 60;
const MIX_MINUTES = 10;

export default () => {
  const mixWater = (device: Device.FirebaseDevice, mqttClient: MqttClient) => {
    const startMixingTopic = Watermixer.getStartMixingTopic(device.uid);
    const request: Device.Request = {
      correlationData: getRandomShortUid(),
    };
    mqttClient.subscribe(startMixingTopic.response);

    const createListener = () =>
      mqttClient.once('message', (topic, payload, packet) => {
        console.log('Received message', { topic, payload, packet });
        const responseRequest = JSON.parse(payload.toString()) as Device.Request;

        if (request.correlationData === responseRequest.correlationData) {
          console.log('That was response for previous request');
          mqttClient.unsubscribe(startMixingTopic.response);
          const finishMixTimestamp = Date.now() + MILLIS_IN_SECOND * SECOND_IN_MINUTE * MIX_MINUTES;
          updateDevice({
            ...device,
            data: {
              finishMixTimestamp,
            },
          });
          return;
        }
        createListener();
      });

    createListener();

    mqttClient.publish(startMixingTopic.request, JSON.stringify(request));
  };

  return {
    mixWater,
  };
};
