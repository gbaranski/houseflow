import chalk from 'chalk';
import express from 'express';
import { IncomingMessage } from 'http';

const log = console.log;

const serverName = 'controlhome-api';

export function logRequest(req: express.Request, res: express.Response): void {
  res.addListener('close', () => {
    log(
      chalk.blueBright(
        `${chalk.green(serverName)} | ${req.method} ${
          req.path
        } ${chalk.yellowBright(res.statusCode)}`,
      ),
    );
  });
}

export function logPingPong(deviceName: string, isPing: boolean): void {
  log(
    chalk.cyanBright(
      `${chalk.green(serverName)} | ${deviceName} ${isPing ? 'PING' : 'PONG'}`,
    ),
  );
}

export function logMissing(what: string): void {
  log(chalk.magenta(`${chalk.green(serverName)} | Missing ${what}`));
}

export function logInvalid(what: string): void {
  log(chalk.magenta(`${chalk.green(serverName)} | Invalid ${what}`));
}

export function logAdded(what: string): void {
  log(chalk.magenta(`${chalk.green(serverName)} | Added ${what}`));
}

export function logError(what: string): void {
  log(chalk.magenta(`${chalk.green(serverName)} | Error ${what}`));
}

export function logSocketConnection(
  req: IncomingMessage,
  deviceName: string | string[],
): void {
  log(
    chalk.blueBright(
      `${chalk.green(serverName)} | [WS] Connect ${deviceName} IP: ${
        req.socket.remoteAddress
      } PORT: ${req.socket.remotePort}`,
    ),
  );
}
