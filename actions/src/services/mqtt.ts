import { getRandomShortID } from '@/utils';
import { SmartHomeV1ExecuteResponseCommands } from 'actions-on-google';
import { DeviceOfflineException } from '@/types';
import mqtt from 'mqtt';

const mqttClient = mqtt.connect('mqtt://emqx', {
  clientId: 'server_actions-1',
});
mqttClient.on('connect', () => {
  console.log('Successfully connected ');
});
mqttClient.on('error', (err) => {
  console.log(`MQTT error occured ${err}`);
});

interface DeviceResponse extends SmartHomeV1ExecuteResponseCommands {
  correlationData: string;
}

const sendMessage = ({
  topic,
  payload,
}: {
  payload: Object;
  topic: string;
}) => {
  const topicPair = {
    request: topic + '/request',
    response: topic + '/response',
  };
  mqttClient.subscribe(topicPair.response);

  const correlationData = getRandomShortID();

  const request = {
    ...payload,
    correlationData: correlationData,
  };

  return new Promise<Object>((resolve, reject) => {
    let completed = false;

    const listenerCallback = (
      msgTopic: string,
      payload: Buffer,
      packet: mqtt.Packet,
    ) => {
      const responseRequest = JSON.parse(
        payload.toString('utf8'),
      ) as DeviceResponse;

      if (
        msgTopic === topicPair.response &&
        correlationData === responseRequest.correlationData
      ) {
        mqttClient.unsubscribe(topicPair.response);
        mqttClient.removeListener('message', listenerCallback);
        completed = true;
        resolve(responseRequest);
      }
    };

    mqttClient.on('message', listenerCallback);

    mqttClient.publish(topicPair.request, JSON.stringify(request));
    setTimeout(() => {
      if (completed) return;
      mqttClient.unsubscribe(topicPair.response);
      mqttClient.removeListener('message', listenerCallback);
      reject(DeviceOfflineException);
    }, 3000);
  });
};

export const sendCommand = async (
  deviceID: string,
  cmd: string,
  params: object,
): Promise<SmartHomeV1ExecuteResponseCommands> => {
  try {
    const res = await sendMessage({
      topic: `${deviceID}/commands/${cmd}`,
      payload: { params },
    });
    return res as SmartHomeV1ExecuteResponseCommands;
  } catch (e) {
    if (e instanceof DeviceOfflineException) {
      return {
        ids: [deviceID],
        status: 'OFFLINE',
      };
    } else {
      return {
        ids: [deviceID],
        status: 'ERROR',
        errorCode: e.message,
      };
    }
  }
};
