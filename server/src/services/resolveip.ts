import geoip from 'geoip-lite';
import { IncomingMessage } from 'http';

export function getIpStr(req: IncomingMessage): string {
  const ip =
    req.headers['X-Forwarded-For'] ||
    req.headers['X-Real-IP '] ||
    req.connection.remoteAddress ||
    '';
  return ip instanceof Array ? ip[0] : ip;
}

export function getCountryStr(ip: string): string {
  const geo = geoip.lookup(ip);
  return geo ? geo.country : 'unknown';
}
