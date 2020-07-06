/* eslint-disable no-console */
import express from 'express';

export function getIp(req: express.Request): string | string[] | undefined {
  return (
    req.headers['cf-connecting-ip'] ||
    req.headers['x-forwarded-for'] ||
    req.connection.remoteAddress
  );
}
export function getIpStr(req: express.Request): string {
  return String(getIp(req));
}

export async function fetchURL(
  url: string,
  path: string,
  headers: Headers,
): Promise<number> {
  let statusCode = 0;
  await fetch(`${url}${path}`, {
    method: 'POST',
    headers,
  })
    .then(_data => {
      console.log('Success:', _data.status);
      statusCode = _data.status;
    })
    .catch(() => {
      console.error(`Error while fetching ${path}`);
      statusCode = 503;
    });
  return statusCode;
}
