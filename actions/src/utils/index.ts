import { Headers } from 'actions-on-google';

export const extractJWTToken = (headers: Headers): string => {
  const header = headers.authorization;
  if (!header) throw new Error("Headers doesn't include authorization header");
  if (header instanceof Array)
    throw new Error('Authorization header is type of array');

  const splitted = header.split(' ');
  if (splitted.length != 2) throw new Error('Authorization header is invalid');
  return splitted[1];
};

export const getRandomShortID = (): string =>
  Math.random().toString(36).substr(2, 9);
