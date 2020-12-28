import http from 'http';
import app from './app';
import chalk from 'chalk';

const httpServer = http.createServer(app);

httpServer.listen(80, '0.0.0.0', () => {
  console.log(chalk.yellowBright(`Successfully started at port 80`));
});
