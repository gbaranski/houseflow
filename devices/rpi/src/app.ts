import { MqttClient } from 'mqtt';
import { Topic } from '@houseflow/types';
import { startMixing } from './services/relay';

const { DEVICE_UID } = process.env;

export const onConnection = (mqtt: MqttClient) => {
  console.log('Initialized connection with MQTT');

  const basicTopic = `${DEVICE_UID}/trigger1`;
  const startMixTopic: Topic = {
    request: `${basicTopic}/request`,
    response: `${basicTopic}/response`,
  };

  mqtt.subscribe(startMixTopic.request);

  mqtt.on('message', (topic, payload, packet) => {
    console.log({ topic, payload, packet });

    switch (topic) {
      case startMixTopic.request:
        startMixing();
        sendRequestResponse(startMixTopic.response, payload);
        break;
      default:
        console.log('Unrecognized topic');
        break;
    }
  });

  const sendRequestResponse = (topic: string, payload: Buffer) => {
    mqtt.publish(topic, payload);
  };
};
