import { Device } from '@gbaranski/types';
import { PubSub } from '@google-cloud/pubsub';

const pubSubClient = new PubSub();

export async function createDeviceTopic(uid: string) {
  await pubSubClient.createTopic(uid);
  console.log(`Topic ${uid} created.`);
}

export async function removeDeviceTopic(uid: string) {
  await pubSubClient.topic(uid).delete();
  console.log(`Topic ${uid} deleted.`);
}

export async function publishDeviceData(device: Device.ActiveDevice) {
  const dataBuffer = Buffer.from(JSON.stringify(device));
  const messageId = await pubSubClient.topic(device.uid).publish(dataBuffer);
  console.log(`Message ${messageId} published.`);
}
