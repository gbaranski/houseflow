import http from 'http';
import app from './app';
import chalk from 'chalk';

const PORT = 80;

const httpServer = http.createServer(app);

httpServer.listen(PORT, '0.0.0.0', () => {
  console.log(chalk.yellowBright(`Successfully started at port ${PORT}`));
});
