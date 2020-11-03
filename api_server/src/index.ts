import http from 'http';
import app from './app';
import chalk from 'chalk';

const PORT = 80;

const httpServer = http.createServer(app);

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
httpServer.listen(PORT, '0.0.0.0', () => {
  console.log(chalk.yellowBright(`Successfully started at port ${PORT}`));
});
