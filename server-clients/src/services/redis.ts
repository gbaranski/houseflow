import redis from 'redis';
import { Device } from '@gbaranski/types';

export let activeDevices: Device.ActiveDevice[] = [];

export const redisSubscriber = redis.createClient();
redisSubscriber.subscribe('device_data');
redisSubscriber.on('message', (channel, message) =>
  handleMessage(channel, message),
);

const handleMessage = (channel: string, message: string) => {
  if (channel !== 'device_data') throw new Error('Unrecognized channel');

  const activeDevice = JSON.parse(message) as Device.ActiveDevice;

  activeDevices = activeDevices
    .filter((device) => activeDevice.uid !== device.uid)
    .concat(activeDevice);
  console.log({ activeDevice, activeDevices });
};
