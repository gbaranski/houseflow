import Device from '@/devices';
import {
  Device as DeviceType,
  CloudTopics,
  AnyDeviceData,
} from '@gbaranski/types';
import { Message, PubSub } from '@google-cloud/pubsub';

const pubSubClient = new PubSub();

export const subscribeToRequests = () => {
  const subscription = pubSubClient.subscription(CloudTopics.DEVICE_REQUEST);
  subscription.on('message', onRequestMessage);
  subscription.on('error', onSubError);
};
const onSubError = (message: Message) => {
  console.log(`Error on data topic: ${message.data}`);
  message.ack();
};

const onRequestMessage = async (message: Message) => {
  console.log(`RECEIVE TIME: ${Date.now()}`);
  const request = JSON.parse(
    message.data.toString(),
  ) as DeviceType.RequestDevice;
  console.log(
    `Received request: ${request.topic.name} to ${request.topic.uid}`,
  );

  if (!request.topic.uid) throw new Error('Device uid is not defined');

  const deviceObjPromise = new Promise<Device<AnyDeviceData>>(
    (resolve, reject) => {
      const found = Device.currentDeviceObjects.find(
        (deviceObject) => request.topic.uid === deviceObject.firebaseDevice.uid,
      );
      if (found) resolve(found);
      if (!found) reject("Couldn't find device object");
    },
  );
  Device.sendRequest(request, deviceObjPromise);
  message.ack();
};

export async function publishDeviceData(device: DeviceType.ActiveDevice) {
  const dataBuffer = Buffer.from(JSON.stringify(device));
  const messageId = await pubSubClient
    .topic(CloudTopics.DEVICE_DATA)
    .publish(dataBuffer);
  console.log(`Data for ${device.uid} published ID ${messageId}.`);
}

export async function publishDeviceDisconnect(device: DeviceType.ActiveDevice) {
  const dataBuffer = Buffer.from(JSON.stringify(device));
  const messageId = await pubSubClient
    .topic(CloudTopics.DEVICE_DISCONNECT)
    .publish(dataBuffer);
  console.log(`Disconnect for ${device.uid} published ID ${messageId}.`);
}
