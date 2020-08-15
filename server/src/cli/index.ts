import chalk from 'chalk';
import express from 'express';
import { IncomingMessage } from 'http';
import { getIpStr } from '@/services/resolveip';

const log = console.log;
type Target = 'client' | 'device';

export function logRequest(req: express.Request, res: express.Response): void {
  res.once('finish', () => {
    log(
      chalk.blueBright(
        `${req.method} ${req.path} ${chalk.yellowBright(
          res.statusCode,
        )} ${getIpStr(req)} ${0}ms`,
      ),
    );
  });
}

export function logMissing(what: string): void {
  log(chalk.magenta(`Missing ${what}`));
}

export function logInvalid(what: string): void {
  log(chalk.magenta(`Invalid ${what}`));
}

export function logAdded(what: string): void {
  log(chalk.magenta(`Added ${what}`));
}

export function logError(what: string): void {
  log(chalk.magenta(`Error ${what}`));
}

const getSocketPrefix = (target: Target) =>
  target === 'client' ? '[WSClient]' : '[WSDevice]';

export function logSocketConnection(
  req: IncomingMessage,
  target: Target,
  deviceName?: string | string[],
): void {
  log(
    chalk.blueBright(
      `${getSocketPrefix(target)} New connection ${deviceName ||
        'someone'} IP: ${getIpStr(req)} PORT: ${req.socket.remotePort}`,
    ),
  );
}

export function logSocketPingPong(
  deviceName: string,
  text: 'PING' | 'PONG',
  target: Target,
): void {
  log(
    chalk.cyanBright(
      `${getSocketPrefix(
        target,
      )} ${deviceName} ${text} ${new Date().getSeconds()}`,
    ),
  );
}

export function logSocketAttempt(
  req: IncomingMessage,
  deviceName: string | string[],
  target: Target,
): void {
  log(
    chalk.blueBright(
      `${getSocketPrefix(target)} Attempt connect ${deviceName} IP: ${
        req.socket.remoteAddress
      } PORT: ${req.socket.remotePort}`,
    ),
  );
}

export function logIntervalStop(
  deviceName: string,
  uid: string,
  reason: string,
  target: Target,
): void {
  log(
    chalk.redBright(
      `${getSocketPrefix(
        target,
      )} Stopped connection, caused by ${reason}. Name: ${deviceName} UID: ${uid}`,
    ),
  );
}
export function logSocketError(
  deviceName: string,
  uid: string,
  message: string,
  target: Target,
): void {
  log(
    chalk.redBright(
      `${getSocketPrefix(
        target,
      )} Error with websocket ${deviceName} UID: ${uid} | Error: ${message} `,
    ),
  );
}
