import mqtt, { MqttClient } from 'mqtt';

const username = process.env.DEVICE_API_USERNAME;
const password = process.env.DEVICE_API_PASSWORD;

if (!username || !password)
  throw new Error('Username or password is not defined in .env, read docs');

export const createMqttClient = (): MqttClient => {
  const mqttClient = mqtt.connect('mqtt://emqx', {
    username,
    password,
    clientId: `server_device-1`,
  });
  mqttClient.on('connect', () => {
    console.log('Successfully connected ');
  });
  mqttClient.on('error', (err) => {
    console.log(`MQTT error occured ${err}`);
  });
  return mqttClient;
};
