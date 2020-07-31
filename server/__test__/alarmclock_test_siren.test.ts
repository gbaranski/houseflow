import fetch from 'node-fetch';
import { username, password } from './globals';

describe('test siren on alarmclock', () => {
  it('attempting to test alarmclock siren with invalid credentials', async () => {
    const res = await fetch('http://localhost:8000/alarmclock/testSiren', {
      method: 'POST',
      headers: {
        username: 'randomUsername',
        password: 'randomPassword',
      },
    });
    expect(res.status).toEqual(401);
  });
  it('attempting to test alarmclock siren with valid credentials', async () => {
    const res = await fetch('http://localhost:8000/alarmclock/testSiren', {
      method: 'POST',
      headers: {
        username: username,
        password: password,
      },
    });
    expect(res.status).toEqual(201);
  });
});
