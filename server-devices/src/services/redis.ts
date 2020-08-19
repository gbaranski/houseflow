import redis from 'redis';
import { Device } from '@gbaranski/types';

export const redisPublisher = redis.createClient();
console.log('Initialized redis publisher');

export const publishDeviceData = (deviceData: Device.ActiveDevice) => {
  redisPublisher.publish('device_data', JSON.stringify(deviceData));
};
