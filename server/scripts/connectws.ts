import WebSocket from 'ws';
import readline from 'readline';
import fetch, { Headers } from 'node-fetch';

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

const prodApiUrl = 'https://api.gbaranski.com/api/getClientToken';
const devApiUrl = `http://localhost:${process.env.HTTP_PORT}/api/getClientToken`;
const devSocketUrl = `ws://localhost:${process.env.WS_CLIENT_PORT}`;
const prodSocketUrl = `wss://ws.gbaranski.com:443`;

(async () => {
  if (!process.env.GBARANSKI) {
    throw new Error('No process.env.GBARANSKI');
  }
  const headers = new Headers();
  const username = 'gbaranski';
  const password = process.env[username.toUpperCase()] as string;

  headers.append('username', username);
  headers.append('password', password);
  const res = fetch(devApiUrl, {
    headers,
  });

  const resText = await (await res).text();
  console.log(resText);

  const ws = new WebSocket(devSocketUrl, {
    headers: { token: resText },
  });
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
          return;
        }
        ws.send(answer);
        recursiveAsyncReadLine();
      });
    };
    recursiveAsyncReadLine();
  });
})();
