import http from 'http';
import app from './app';
import chalk from 'chalk';
import '@/services/mqtt';
import { log } from './services/logging';

const PORT = 80;

const httpServer = http.createServer(app);

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
httpServer.listen(PORT, '0.0.0.0', () => {
  log(chalk.yellowBright(`Successfully started at port ${PORT}`));
});
