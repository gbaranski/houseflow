import express from 'express';

export function getIp(req: express.Request) {
  return (
    req.headers['cf-connecting-ip'] ||
    req.headers['x-forwarded-for'] ||
    req.connection.remoteAddress
  );
}
