import { getRandomShortUid } from '@/utils';
import { Device, Topic, Exceptions } from '@houseflow/types';
import chalk from 'chalk';
import mqtt from 'mqtt';
import { createMqttClient } from './mqttClient';

const mqttClient = createMqttClient();

const generateTopic = (uid: string, action: Device.Action): Topic => {
  const basicTopic = `${uid}/action${action.id}`;
  return {
    request: `${basicTopic}/request`,
    response: `${basicTopic}/response`,
  };
};

export type SendMessageStatus =
  | Exceptions.SUCCESS
  | Exceptions.DEVICE_TIMED_OUT;
export const sendDeviceMessage = (
  uid: string,
  action: Device.Action,
): Promise<SendMessageStatus> => {
  const topic = generateTopic(uid, action);

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
            `Request to ${uid} with action ${action.name} completed successfully`,
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
          `Request to ${uid} with action ${action.name} failed due to timeout`,
        ),
      );
      mqttClient.unsubscribe(topic.response);
      mqttClient.removeListener('message', listenerCallback);
      resolve(Exceptions.DEVICE_TIMED_OUT);
    }, 3000);
  });
};

export default mqttClient;
