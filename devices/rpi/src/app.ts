import { SmartHomeV1ExecuteResponseCommands } from 'actions-on-google';
import { mqttClient } from '.';

const { DEVICE_UID } = process.env;
if (!DEVICE_UID) throw new Error('DEVICE_UID not defined');

interface DeviceRequest {
  correlationData: string;
  params: any;
}

interface DeviceResponse extends SmartHomeV1ExecuteResponseCommands {
  correlationData: string;
}

let currentState = {
  online: true,
  on: true,
};

// It can be used for example when someone turns on/off light by physical switch
const publishState = () => {
  console.log('Publishing state: ', currentState);
  mqttClient.publish(`${DEVICE_UID}/reportState`, JSON.stringify(currentState));
};

export const onConnection = () => {
  console.log('Initialized connection with MQTT');
  publishState();

  const OnOffTopic = {
    request: `${DEVICE_UID}/commands/OnOff/request`,
    response: `${DEVICE_UID}/commands/OnOff/response`,
  };

  setTimeout(() => {
    currentState.on = false;
    publishState();
  }, 2000);

  mqttClient.subscribe(OnOffTopic.request);

  mqttClient.on('message', (topic, payload, packet) => {
    console.log({ topic, payload, packet });

    switch (topic) {
      case OnOffTopic.request:
        const onOffRequest = JSON.parse(
          payload.toString('utf8'),
        ) as DeviceRequest;
        console.log(`Changing lights state to ${onOffRequest.params.on}!`);
        currentState.on = onOffRequest.params.on;

        const response: DeviceResponse = {
          correlationData: onOffRequest.correlationData,
          ids: [DEVICE_UID],
          status: 'SUCCESS',
          states: {
            ...currentState,
          },
        };
        mqttClient.publish(OnOffTopic.response, JSON.stringify(response));
        break;
      default:
        console.log('Unrecognized topic');
        break;
    }
  });
};
