import chalk from 'chalk';
import express from 'express';
import { IncomingMessage } from 'http';
import { getIpStr } from '../helpers';

const log = console.log;

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

export function logPingPong(deviceName: string, isPing: boolean): void {
  log(chalk.cyanBright(`[WS] ${deviceName} ${isPing ? 'PING' : 'PONG'}`));
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

export function logSocketConnection(
  req: IncomingMessage,
  deviceName: string | string[],
): void {
  log(
    chalk.blueBright(
      `[WS] Connect ${deviceName} IP: ${req.socket.remoteAddress} PORT: ${req.socket.remotePort}`,
    ),
  );
}
