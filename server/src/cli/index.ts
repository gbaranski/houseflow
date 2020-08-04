import chalk from 'chalk';
import express from 'express';
import { IncomingMessage } from 'http';
import { getIpStr } from '@/services/resolveip';

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

export function logSocketAttempt(
  req: IncomingMessage,
  deviceName: string | string[],
): void {
  log(
    chalk.blueBright(
      `[WS] Attempt connect ${deviceName} IP: ${req.socket.remoteAddress} PORT: ${req.socket.remotePort}`,
    ),
  );
}

export function logIntervalStop(
  deviceName: string,
  uid: string,
  reason: string,
): void {
  log(
    chalk.redBright(
      `[WS] Stopped connection, caused by ${reason}. Name: ${deviceName} UID: ${uid}`,
    ),
  );
}
export function logSocketError(
  deviceName: string,
  uid: string,
  message: string,
): void {
  log(
    chalk.redBright(
      `[WS] Error with websocket ${deviceName} UID: ${uid} | Error: ${message} `,
    ),
  );
}
