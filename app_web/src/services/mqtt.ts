import { getRandomShortUid } from '@/utils/utils';
import mqtt from 'mqtt';

const BROKER_DEV_URL = 'wss://localhost:8084/mqtt';
const BROKER_PROD_URL = 'wss://api.gbaranski.com:8084/mqtt';

const BROKER_URL = process.env.NODE_ENV === 'development' ? BROKER_DEV_URL : BROKER_PROD_URL;

export const connectMqtt = (token: string, uid: string) => {
  return new Promise<mqtt.MqttClient>((resolve, reject) => {
    const client = mqtt.connect(BROKER_PROD_URL, {
      clientId: `web_ + ${getRandomShortUid()}`,
      username: uid,

      password: token,
    });
    client.on('connect', () => {
      resolve(client);
    });
    client.on('error', reject);
  });
};
