import { Device } from '@gbaranski/types';
import { PubSub, Message } from '@google-cloud/pubsub';
import { io } from '..';
import { updateDeviceData } from './websocket';

export let activeDevices: Device.ActiveDevice[] = [];

const pubSubClient = new PubSub();

enum Topics {
  DEVICE_DATA = 'device_data',
  DEVICE_DISCONNECT = 'device_disconnect',
  DEVICE_REQUEST = 'device_request',
}

export async function subscribeToDevicesData() {
  const subscription = pubSubClient.subscription(Topics.DEVICE_DATA);
  subscription.on('message', onDataMessage);
  subscription.on('error', onDataError);
  console.log(`Subscribed to ${Topics.DEVICE_DATA}`);
}

const onDataError = (message: Message) => {
  console.log(`Error on data topic: ${message.data}`);
  message.ack();
};

const onDataMessage = async (message: Message) => {
  console.log(`Received data ${message.id}`);
  console.log(`Data: ${message.data.toString()}`);
  const activeDevice = JSON.parse(
    message.data.toString(),
  ) as Device.ActiveDevice;
  message.ack();

  activeDevices = activeDevices
    .filter((device) => activeDevice.uid !== device.uid)
    .concat(activeDevice);
  updateDeviceData(io, activeDevice);
};

export const publishRequest = async (request: Device.RequestDevice) => {
  const dataBuffer = Buffer.from(JSON.stringify(request));
  const messageId = await pubSubClient
    .topic(Topics.DEVICE_REQUEST)
    .publish(dataBuffer);
  console.log(
    `Request for ${request.topic.uid} ${request.topic.name} published ID ${messageId}.`,
  );
};
