import WebSocket from 'ws';

if (!process.env.WS_DEVICE_PORT)
  throw new Error('WS Device port is not defined in .env');

test('Attempt connecting websocket without JWT Token', done => {
  const ws = new WebSocket(`ws://localhost:${process.env.WS_DEVICE_PORT}`);

  ws.addEventListener('error', () => {
    done();
  });
});
