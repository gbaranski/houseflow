import redis from 'redis';
import { Client } from '@gbaranski/types';

const redisPublisher = redis.createClient('redis://redis:6379');
redisPublisher.on('connect', () => console.log('Initialized redis publisher'));

export const publishRequest = (request: Client.Request) => {
  redisPublisher.publish('request', JSON.stringify(request));
};
