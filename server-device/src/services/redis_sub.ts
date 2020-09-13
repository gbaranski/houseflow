import redis from 'redis';
import { Device as DeviceType } from '@gbaranski/types';
import { SubChannel } from '@/types';
import Device from '@/devices';

export const redisSubscriber = redis.createClient('redis://redis:6379');
redisSubscriber.on('connect', () => console.log('Initialized redis subsciber'));
redisSubscriber.subscribe('request');

redisSubscriber.on('message', (channel, message) => {
  const targetChannel = channels.find((_channel) => _channel.name === channel);
  if (!targetChannel) throw new Error('Channel unrecognized');
  targetChannel.handle(message);
});

const handleRequest = (message: string) => {
  const request = JSON.parse(message) as DeviceType.RequestDevice;
  if (!request.topic.uid) throw new Error('Device uid is not defined');

  const deviceObj = Device.currentDeviceObjects.find(
    (devObj) => request.topic.uid === devObj.firebaseDevice.uid,
  );
  if (!deviceObj) throw new Error('Couldnt find device object');
  deviceObj.requestDevice(request);
};

const channels: SubChannel[] = [
  {
    name: 'request',
    handle: handleRequest,
  },
];
