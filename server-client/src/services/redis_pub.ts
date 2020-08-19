import redis from 'redis';
import { Client } from '@gbaranski/types';

const redisPublisher = redis.createClient();

export const publishRequest = (request: Client.Request) => {
  redisPublisher.publish('request', JSON.stringify(request));
};
