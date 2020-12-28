import redis from 'redis';

const client = redis.createClient({
  host: 'redis',
  db: 0,
});

// Returns userID for corresponding tokenUUID, throws if doesn't exist
export const fetchTokenUUID = (tokenUUID: string): Promise<string> => {
  return new Promise<string>((resolve, reject) => {
    client.get(tokenUUID, (err, reply) => {
      if (err) reject(err);
      if (!reply) {
        return reject(new Error('Token not found'));
      }
      resolve(reply);
    });
  });
};
