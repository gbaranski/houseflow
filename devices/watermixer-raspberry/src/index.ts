import mqtt from 'mqtt';
import { onConnection } from './app';

const { DEVICE_UID, DEVICE_SECRET, MQTT_URL } = process.env;

if (!DEVICE_UID || !DEVICE_SECRET || !MQTT_URL) {
  throw new Error('DEVICE_UID or DEVICE_SECRET are not defined in .env');
}

const mqttClient = mqtt.connect(MQTT_URL);
mqttClient.on('connect', () => onConnection(mqttClient))
