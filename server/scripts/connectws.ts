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
  headers.append('device', 'ALARMCLOCK');
  headers.append('token', process.env.ALARMCLOCK || '');
  // process.env.NODE_TLS_REJECT_UNAUTHORIZED = '0';
  const res = fetch(`https://api.gbaranski.com/api/getToken`, {
    headers,
  });

  const resText = await (await res).text();
  console.log(resText);

  const ws = new WebSocket(`wss://ws.gbaranski.com:443/`, {
    headers: { token: resText },
  });
  ws.on('open', async () => {
    console.log('Logged in');
    ws.on('message', console.log);
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
