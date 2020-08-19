import redis from 'redis';
import { Device } from '@gbaranski/types';

export const redisPublisher = redis.createClient('redis://redis:6379');
console.log('Initialized redis publisher');

export const publishDeviceData = (device: Device.ActiveDevice) => {
  redisPublisher.publish('device_data', JSON.stringify(device));
};

export const publishDeviceDisconnect = (device: Device.ActiveDevice) => {
  redisPublisher.publish('device_disconnect', JSON.stringify(device));
};
