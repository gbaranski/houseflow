import { connect, MqttClient } from 'mqtt';

const BROKER_URL =
  process.env.NODE_ENV === 'development'
    ? 'http://localhost:8083'
    : 'mqtt://api.gbaranski.com:443/wsc';

console.log({ processenv: process.env.NODE_ENV });

export const connectMqtt = (token: string, uid: string) => {
  return new Promise<MqttClient>((resolve, reject) => {
    const client = connect(BROKER_URL, {
      clientId: 'app_web',
      username: uid,
      password: token,
    });
    client.on('connect', () => {
      resolve(client);
    });
    client.on('error', reject);
  });
};
