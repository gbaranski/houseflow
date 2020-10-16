import { getRandomShortUid } from '@/utils/utils';
import { Device, Topic } from '@houseflow/types';
import mqtt, { MqttClient } from 'mqtt';

const BROKER_DEV_URL = 'wss://localhost:8084/mqtt';
const BROKER_PROD_URL = 'wss://api.gbaranski.com:8084/mqtt';

const BROKER_URL = process.env.NODE_ENV === 'development' ? BROKER_DEV_URL : BROKER_PROD_URL;

export const connectMqtt = (token: string, uid: string) => {
  return new Promise<mqtt.MqttClient>((resolve, reject) => {
    const client = mqtt.connect(BROKER_PROD_URL, {
      clientId: `web_${getRandomShortUid()}`,
      username: uid,

      password: token,
    });
    client.on('connect', () => {
      resolve(client);
    });
    client.on('error', reject);
  });
};

interface SendRequest {
  request: Device.Request;
  topic: Topic;
  mqttClient: MqttClient;
}

export const sendRequest = ({ request, topic, mqttClient }: SendRequest) => {
  mqttClient.subscribe(topic.response);

  return new Promise<void>((resolve, reject) => {
    const createListener = () =>
      mqttClient.once('message', (msgTopic, payload, packet) => {
        console.log('Received message', { topic, payload, packet });
        const responseRequest = JSON.parse(payload.toString()) as Device.Request;

        if (request.correlationData === responseRequest.correlationData) {
          console.log('Received response for previous request');
          mqttClient.unsubscribe(topic.response);
          resolve();
          return;
        }
        createListener();
      });

    mqttClient.once('error', (e) => {
      console.log(new Error(`Rejected send message promise due to ${e.message}`));
      reject(e.message);
    });

    createListener();

    mqttClient.publish(topic.request, JSON.stringify(request));

    setTimeout(() => {
      mqttClient.unsubscribe(topic.response);
      reject(new Error('Timed out! Device might be offline.'));
    }, 3000);
  });
};
