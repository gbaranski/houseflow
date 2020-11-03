import chalk from 'chalk';

export const logServerError = (err: Error): void => {
  console.error(chalk.red(`Server error occured: ${err.message}`));
};
export const logUnhandledError = (err: Error): void => {
  console.debug(chalk.red(`Unhandled error: ${err}`));
  console.log(chalk.red(`Stacktrace: ${err.stack}`));
};
export const logClientAuthError = (err: Error): void => {
  console.info(chalk.redBright(`Client authentication: ${err.message}`));
};

export const logClientError = (err: Error): void => {
  console.info(chalk.yellow(`Client error: ${err.message}`));
};
