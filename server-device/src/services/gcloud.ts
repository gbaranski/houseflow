import { Device } from '@gbaranski/types';
import { PubSub } from '@google-cloud/pubsub';

const pubSubClient = new PubSub();

enum Topics {
  DEVICE_DATA = 'device_data',
  DEVICE_DISCONNECT = 'device_disconnect',
}

export async function publishDeviceData(device: Device.ActiveDevice) {
  const dataBuffer = Buffer.from(JSON.stringify(device));
  const messageId = await pubSubClient
    .topic(Topics.DEVICE_DATA)
    .publish(dataBuffer);
  console.log(`Data for ${device.uid} published ID ${messageId}.`);
}

export async function publishDeviceDisconnect(device: Device.ActiveDevice) {
  const dataBuffer = Buffer.from(JSON.stringify(device));
  const messageId = await pubSubClient
    .topic(Topics.DEVICE_DISCONNECT)
    .publish(dataBuffer);
  console.log(`Disconnect for ${device.uid} published ID ${messageId}.`);
}
