import { MqttClient } from 'mqtt';
import { SmartHomeV1ExecuteResponseCommands } from 'actions-on-google';

const { DEVICE_UID } = process.env;
if (!DEVICE_UID) throw new Error('DEVICE_UID not defined');

interface DeviceRequest {
  correlationData: string;
  params: any;
}

interface DeviceResponse extends SmartHomeV1ExecuteResponseCommands {
  correlationData: string;
}

export const onConnection = (mqtt: MqttClient) => {
  console.log('Initialized connection with MQTT');

  const OnOffTopic = {
    request: `${DEVICE_UID}/commands/OnOff/request`,
    response: `${DEVICE_UID}/commands/OnOff/response`,
  };

  mqtt.subscribe(OnOffTopic.request);

  mqtt.on('message', (topic, payload, packet) => {
    console.log({ topic, payload, packet });

    switch (topic) {
      case OnOffTopic.request:
        const request = JSON.parse(payload.toString('utf8')) as DeviceRequest;

        const response: DeviceResponse = {
          ids: [DEVICE_UID],
          status: 'SUCCESS',
          states: {
            on: request.params.on,
          },
          correlationData: request.correlationData,
        };
        sendRequestResponse(OnOffTopic.response, response);
        mqtt.publish(OnOffTopic.response, JSON.stringify(response));
        console.log(`Changing lights state to ${request.params.on}!`);
        break;
      default:
        console.log('Unrecognized topic');
        break;
    }
  });

  const sendRequestResponse = (topic: string, payload: DeviceResponse) => {
    mqtt.publish(topic, JSON.stringify(payload));
  };
};
