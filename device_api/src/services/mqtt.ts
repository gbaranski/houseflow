import { getRandomShortUid } from '@/utils';
import { Device, Topic, Client, Exceptions } from '@houseflow/types';
import chalk from 'chalk';
import mqtt from 'mqtt';
import { createMqttClient } from './mqttClient';

const mqttClient = createMqttClient();

const generateTopic = (request: Client.DeviceRequest['device']): Topic => {
  const basicTopic = `${request.uid}/${request.action}${request.gpio}`;
  return {
    request: `${basicTopic}/request`,
    response: `${basicTopic}/response`,
  };
};

type SendMessageStatus = Exceptions.SUCCESS | Exceptions.DEVICE_TIMED_OUT;
export const sendDeviceMessage = (
  request: Client.DeviceRequest['device'],
): Promise<SendMessageStatus> => {
  const topic = generateTopic(request);

  mqttClient.subscribe(topic.response);

  const correlationData = getRandomShortUid();
  const deviceRequest: Device.Request = {
    correlationData,
  };

  return new Promise<SendMessageStatus>((resolve, reject) => {
    let completed = false;

    const listenerCallback = (
      msgTopic: string,
      payload: Buffer,
      packet: mqtt.Packet,
    ) => {
      const responseRequest = JSON.parse(payload.toString()) as Device.Response;

      if (
        msgTopic === topic.response &&
        correlationData === responseRequest.correlationData
      ) {
        mqttClient.unsubscribe(topic.response);
        mqttClient.removeListener('message', listenerCallback);
        console.info(
          chalk.greenBright(
            `Request to ${request.uid} with action ${request.action} completed successfully`,
          ),
        );
        completed = true;
        resolve(Exceptions.SUCCESS);
        return;
      }
    };

    mqttClient.on('message', listenerCallback);

    mqttClient.publish(topic.request, JSON.stringify(deviceRequest));
    setTimeout(() => {
      if (completed) return;
      console.info(
        chalk.redBright(
          `Request to ${request.uid} with action ${request.action} failed due to timeout`,
        ),
      );
      mqttClient.unsubscribe(topic.response);
      mqttClient.removeListener('message', listenerCallback);
      resolve(Exceptions.DEVICE_TIMED_OUT);
    }, 3000);
  });
};

export default mqttClient;
