import geoip from 'geoip-lite';
import { IncomingMessage } from 'http';
import WebSocket from 'ws';

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

export const validateDeviceMessage = (message: WebSocket.Data): void => {
  if (
    message instanceof Buffer ||
    message instanceof ArrayBuffer ||
    message instanceof Array
  )
    throw new Error('Cannot handle Buffer type');
  const parsedResponse = JSON.parse(message) as { ok: boolean };
  if (!parsedResponse.ok) throw new Error('Response is not okay');
};
