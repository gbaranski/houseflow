import http from 'http';
import app from './app';
import chalk from 'chalk';

export const PORT = process.env.PORT_API as string;

if (!PORT) throw new Error('PORT is not defined');

const httpServer = http.createServer(app);

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
httpServer.listen(PORT, '0.0.0.0', () => {
  console.log(chalk.yellowBright(`Successfully started at port ${PORT}`));
});
