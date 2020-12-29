import { AnyDeviceData } from '@/database/mongo';
import { getRandomShortID } from '@/utils';
import {
  SmartHomeV1ExecuteResponse,
  SmartHomeV1ExecuteResponseCommands,
} from 'actions-on-google';
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

interface DeviceRequest {
  correlationData: string;
  params: object;
}

interface DeviceResponse extends SmartHomeV1ExecuteResponseCommands {
  correlationData: string;
}

export const sendDeviceMessage = (
  deviceID: string,
  cmd: string,
  params: object,
): Promise<SmartHomeV1ExecuteResponseCommands> => {
  const topic = {
    request: `${deviceID}/commands/${cmd}/request`,
    response: `${deviceID}/commands/${cmd}/response`,
  };

  mqttClient.subscribe(topic.response);

  const correlationData = getRandomShortID();

  const deviceRequest: DeviceRequest = {
    correlationData,
    params,
  };

  return new Promise<SmartHomeV1ExecuteResponseCommands>((resolve, reject) => {
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
        msgTopic === topic.response &&
        correlationData === responseRequest.correlationData
      ) {
        mqttClient.unsubscribe(topic.response);
        mqttClient.removeListener('message', listenerCallback);
        console.info(
          `Request to ${deviceID} with command ${cmd} completed successfully`,
        );
        completed = true;
        resolve(responseRequest);
      }
    };

    mqttClient.on('message', listenerCallback);

    mqttClient.publish(topic.request, JSON.stringify(deviceRequest));
    setTimeout(() => {
      if (completed) return;
      console.info(
        `Request to ${deviceID} with command ${cmd} failed due to timeout`,
      );
      mqttClient.unsubscribe(topic.response);
      mqttClient.removeListener('message', listenerCallback);
      resolve({
        ids: [deviceID],
        status: 'OFFLINE',
      });
    }, 3000);
  });
};
