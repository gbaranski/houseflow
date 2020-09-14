import chalk from 'chalk';
import http from 'http';
import mqtt from 'mqtt';
import { onConnection } from '@/services/mqtt';
import '@/services/gcloud';
import { ON_CONNECTED_TOPIC } from './topics';
import { subscribeToRequests } from '@/services/gcloud';

const PORT = process.env.PORT_DEVICE || 8001;
if (!PORT) throw new Error('Port is not defined in .env');

const requestListener: http.RequestListener = (req, res) => {
  res.writeHead(200);
  res.end('Hello from device zone');
};

const httpServer = http.createServer(requestListener);

const mqttClient = mqtt.connect('mqtt://mosquitto:1883');
mqttClient.on('connect', () => {
  mqttClient.subscribe(ON_CONNECTED_TOPIC);
  console.log('Initialized connection with MQTT');

  mqttClient.on('message', (topic, message) => {
    switch (topic) {
      case ON_CONNECTED_TOPIC:
        onConnection(mqttClient, message);
        break;
    }
  });
});

subscribeToRequests();

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
httpServer.listen(PORT, '0.0.0.0', () =>
  console.log(
    chalk.yellow(`Listening for websocket_devices connection at port ${PORT}`),
  ),
);
