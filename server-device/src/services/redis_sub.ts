import redis from 'redis';
import { Device as DeviceType, Client } from '@gbaranski/types';
import { SubChannel } from '@/types';
import Device from '@/devices';

export const redisSubscriber = redis.createClient();
redisSubscriber.subscribe('request');

redisSubscriber.on('message', (channel, message) => {
  const targetChannel = channels.find((_channel) => _channel.name === channel);
  if (!targetChannel) throw new Error('Channel unrecognized');
  targetChannel.handle(message);
});

const handleRequest = (message: string) => {
  const request = JSON.parse(message) as Client.Request;
  if (!request.deviceUid) throw new Error('Device uid is not defined');

  const deviceObj = Device.currentDeviceObjects.find(
    (devObj) => request.deviceUid === devObj.firebaseDevice.uid,
  );
  if (!deviceObj) throw new Error('Couldnt find device object');
  if (request.requestType === 'CONNECTIONS')
    throw new Error('This request shouldnt go there!');
  deviceObj.requestDevice(request);
};

const channels: SubChannel[] = [
  {
    name: 'request',
    handle: handleRequest,
  },
];
