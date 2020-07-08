/* eslint-disable no-console */
import express from 'express';
import fetch, { Headers } from 'node-fetch';

export function getIpStr(req: express.Request): string {
  return String(req.get('cf-connecting-ip') || req.connection.remoteAddress);
}

export function getCountryStr(req: express.Request): string {
  return String(req.header('Cf-Ipcountry'));
}

export async function fetchURL(
  url: string,
  path: string,
  headers?: Headers,
): Promise<number> {
  let statusCode = 0;
  await fetch(`${url}${path}`, {
    method: 'POST',
    headers: headers ? headers : new Headers(),
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
