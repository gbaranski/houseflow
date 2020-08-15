import WebSocket from 'ws';

if (!process.env.WS_CLIENT_PORT)
  throw new Error('WS Device port is not defined in .env');

test('Attempt to connect without token', done => {
  const ws = new WebSocket(`ws://localhost:${process.env.WS_CLIENT_PORT}`);

  ws.addEventListener('error', () => {
    done();
  });
});
