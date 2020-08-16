import WebSocket from 'ws';

if (!process.env.PORT) throw new Error('PORT is not defined in .env');

test('Attempt connection without JWT token and any data', (done) => {
  new WebSocket(`ws://localhost:${process.env.PORT}`).on('error', (e) => {
    expect(e.message).toEqual('Unexpected server response: 401');
    done();
  });
});

test('Attempt connection without JWT token', (done) => {
  new WebSocket(`ws://localhost:${process.env.PORT}`, {
    headers: { deviceType: 'testDevice', uid: 'someUid', secret: 'someSecret' },
  }).on('error', (e) => {
    expect(e.message).toEqual('Unexpected server response: 401');
    done();
  });
});
