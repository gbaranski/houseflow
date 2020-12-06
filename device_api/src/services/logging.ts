import chalk from 'chalk';

const logDate = (): string => {
  const now = new Date();
  const fixDate = (e: number) => e.toString().padStart(2, '0');

  return (
    [now.getFullYear(), now.getMonth(), now.getDate()].map(fixDate).join('/') +
    ' ' +
    [now.getHours(), now.getMinutes(), now.getSeconds()].map(fixDate).join(':')
  );
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const log = (...data: any[]): void =>
  console.log(logDate, ' - ', ...data);

export const logServerError = (err: Error, uid?: string): void => {
  log(
    chalk.redBright(
      `Server error occured: ${err.message} ${uid ? `UID: ${uid}` : ''} `,
    ),
  );
};
export const logUnhandledError = (err: Error): void => {
  log(chalk.redBright(`Error: ${err}`));
};
export const logClientAuthError = (err: Error, uid?: string): void => {
  log(
    chalk.redBright(
      `Client authentication: ${err.message} ${uid ? `UID: ${uid}` : ''} `,
    ),
  );
};

export const logClientError = (err: Error): void => {
  log(chalk.yellow(`Client error: ${err.message}`));
};
