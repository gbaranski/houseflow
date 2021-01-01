import mqtt from 'mqtt';
import { onConnection } from './app';

const { DEVICE_UID, DEVICE_SECRET, MQTT_URL } = process.env;

if (!DEVICE_UID || !DEVICE_SECRET || !MQTT_URL) {
  throw new Error(
    'DEVICE_UID or DEVICE_SECRET or MQTT_URL are not defined in .env',
  );
}

export const mqttClient = mqtt.connect(MQTT_URL, {
  clientId: 'device_' + Math.random().toString(16).substr(2, 8),
  username: DEVICE_UID,
  password: DEVICE_SECRET,
  protocolVersion: 4,
});

mqttClient.on('error', console.log);

mqttClient.on('connect', () => onConnection());
