import WebSocket from 'ws';
import readline from 'readline';

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

const devSocketUrl = `ws://localhost:${process.env.WS_CLIENT_PORT}`;
const prodSocketUrl = `wss://ws.gbaranski.com:443`;

(async () => {
  if (!process.env.GBARANSKI) {
    throw new Error('No process.env.GBARANSKI');
  }
  const ws = new WebSocket(devSocketUrl);
  ws.on('open', async () => {
    console.log('Logged in');
    ws.on('message', console.log);
    ws.on('ping', ws.pong);
    ws.on('pong', ws.ping);

    const recursiveAsyncReadLine = () => {
      rl.question('Command: ', answer => {
        if (answer == 'exit' || answer == '^C') {
          console.log('Exiting');
          process.exit(1);
        }
        ws.send(answer);
        recursiveAsyncReadLine();
      });
    };
    recursiveAsyncReadLine();
  });
})();
