import express from 'express';
import geoip from 'geoip-lite';

export function getIpStr(req: express.Request): string {
  return req.get('X-Forwarded-For') || req.get('X-Real-IP ') || req.ip;
}

export function getCountryStr(ip: string): string {
  const geo = geoip.lookup(ip);
  return geo ? geo.country : 'unknown';
}
