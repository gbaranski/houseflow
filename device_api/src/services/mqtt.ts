import { DeviceRequestDevice } from '@/routes/request/types';
import { MqttDeviceResponse, SendMessageStatus } from '@/types';
import { getRandomShortUid } from '@/utils';
import { Topic } from '@houseflow/types';
import mqtt from 'mqtt';

const username = process.env.DEVICE_API_USERNAME;
const password = process.env.DEVICE_API_PASSWORD;
if (!username || !password)
  throw new Error('Username or password is not defined in .env, read docs');

const mqttClient = mqtt.connect('mqtt://emqx', {
  username,
  password,
  clientId: `device_api-1`,
});

mqttClient.on('connect', (cb) => {
  console.log('Successfully connected ');
});
mqttClient.on('error', (err) => {
  console.log(`MQTT error occured ${err}`);
});

const generateTopic = (request: DeviceRequestDevice): Topic => {
  const basicTopic = `${request.uid}/${request.action}${request.gpio}`;
  return {
    request: `${basicTopic}/request`,
    response: `${basicTopic}/response`,
  };
};

export const sendDeviceMessage = (
  request: DeviceRequestDevice,
): Promise<SendMessageStatus> => {
  const topic = generateTopic(request);

  mqttClient.subscribe(topic.response);

  const sendMessagePromise = new Promise<SendMessageStatus>((resolve) => {
    const correlationData = getRandomShortUid();
    const createListener = () =>
      mqttClient.once('message', (msgTopic, payload, packet) => {
        console.debug('Received message', { msgTopic, payload, packet });

        const responseRequest = JSON.parse(
          payload.toString(),
        ) as MqttDeviceResponse;

        if (
          msgTopic === topic.response &&
          correlationData === responseRequest.correlationData
        ) {
          mqttClient.unsubscribe(topic.response);
          resolve(SendMessageStatus.SUCCESS);
          return;
        }
        createListener();
      });

    createListener();
    mqttClient.publish(topic.request, JSON.stringify(request));
  });

  const timeoutPromise = new Promise<SendMessageStatus>((resolve) => {
    setTimeout(() => resolve(SendMessageStatus.DEVICE_TIMED_OUT), 4000);
  });

  return Promise.race<SendMessageStatus>([timeoutPromise, sendMessagePromise]);
};

export default mqttClient;
