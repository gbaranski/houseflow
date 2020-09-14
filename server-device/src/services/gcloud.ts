import Device from '@/devices';
import { Device as DeviceType, CloudTopics } from '@gbaranski/types';
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
  console.log(`Received request ${message.id}`);
  console.log(`Request: ${message.data.toString()}`);
  const request = JSON.parse(
    message.data.toString(),
  ) as DeviceType.RequestDevice;
  message.ack();

  if (!request.topic.uid) throw new Error('Device uid is not defined');

  const deviceObj = Device.currentDeviceObjects.find(
    (devObj) => request.topic.uid === devObj.firebaseDevice.uid,
  );
  if (!deviceObj) throw new Error('Couldnt find device object');
  deviceObj.requestDevice(request);
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
