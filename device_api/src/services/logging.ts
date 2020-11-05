import chalk from 'chalk';

export const logServerError = (err: Error, uid?: string): void => {
  console.error(
    chalk.redBright(
      `Server error occured: ${err.message} ${uid ? `UID: ${uid}` : ''} `,
    ),
  );
};
export const logUnhandledError = (err: Error): void => {
  console.log(chalk.redBright(`Error: ${err}`));
};
export const logClientAuthError = (err: Error, uid?: string): void => {
  console.log(
    chalk.redBright(
      `Client authentication: ${err.message} ${uid ? `UID: ${uid}` : ''} `,
    ),
  );
};

export const logClientError = (err: Error): void => {
  console.log(chalk.yellow(`Client error: ${err.message}`));
};
