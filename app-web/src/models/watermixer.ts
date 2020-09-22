import { getRandomShortUid } from '@/utils/utils';
import { Watermixer, Device } from '@gbaranski/types';
import { MqttClient } from 'mqtt';

export default () => {
  const mixWater = (uid: string, mqttClient: MqttClient) => {
    const topic = Watermixer.getStartMixingTopic(uid);
    const request: Device.Request = {
      correlationData: getRandomShortUid(),
    };
    mqttClient.subscribe(topic.response);

    const createListener = () =>
      mqttClient.once('message', (topic, payload, packet) => {
        console.log('Received message', { topic, payload, packet });
        const responseRequest = JSON.parse(payload.toString()) as Device.Request;

        if (request.correlationData === responseRequest.correlationData) {
          console.log('That was response for previous request');
          return;
        }
        createListener();
      });

    createListener();

    mqttClient.publish(topic.request, JSON.stringify(request));
  };

  return {
    mixWater,
  };
};
