import WebSocket from 'ws';
import readline from 'readline';
import fetch, { Headers } from 'node-fetch';

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

(async () => {
  if (!process.env.GBARANSKI) {
    throw new Error('No process.env.GBARANSKI');
  }
  const headers = new Headers();
  headers.append('username', 'gbaranski');
  headers.append('password', process.env.GBARANSKI);
  const res = fetch(`http://localhost:${process.env.PORT}/api/getToken`, {
    headers,
  });

  const resText = await (await res).text();
  const ws = new WebSocket(`ws://localhost:${process.env.PORT}`, {
    headers: { token: resText },
  });
  ws.on('open', async () => {
    console.log('Logged in');
    const recursiveAsyncReadLine = function() {
      rl.question('Command: ', function(answer) {
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
