import mqtt from 'mqtt';
import fs from 'fs';
import { onConnection } from './app';

const { DEVICE_UID, DEVICE_SECRET, MQTT_URL } = process.env;

if (!DEVICE_UID || !DEVICE_SECRET || !MQTT_URL) {
  throw new Error('DEVICE_UID or DEVICE_SECRET are not defined in .env');
}

const caCert = fs.readFileSync('../../certs/ca.pem', 'utf-8');
if (!caCert) throw new Error('ca.pem is empty or not exists in certs/ca.pem');

const mqttClient = mqtt.connect('wss://api.gbaranski.com:8084/mqtt', {
  clientId: 'device_' + Math.random().toString(16).substr(2, 8),
  username: DEVICE_UID,
  password: DEVICE_SECRET,
  protocolVersion: 4,
  ca: caCert,
  clean: true,
});

mqttClient.on('error', console.log);

mqttClient.on('connect', () => onConnection(mqttClient));
