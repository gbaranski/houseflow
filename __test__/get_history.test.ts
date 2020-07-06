/* eslint-disable @typescript-eslint/explicit-function-return-type */
import fetch from 'node-fetch';
import { username, password } from './globals';

describe('testing retreiving history', () => {
  it('attempting to get history with no credentials', async () => {
    const res = await fetch('http://localhost:8000/api/getHistory', {
      method: 'GET',
    });
    expect(res.status).toEqual(401);
  });
  it('attempting to get history with invalid credentials', async () => {
    const res = await fetch('http://localhost:8000/api/getHistory', {
      method: 'GET',
      headers: {
        username: 'randomUsername',
        password: 'randomPassword',
      },
    });
    expect(res.status).toEqual(401);
  });
  it('attempting to get history with valid credentials', async () => {
    const res = await fetch('http://localhost:8000/api/getHistory', {
      method: 'GET',
      headers: {
        username: username,
        password: password,
      },
    });
    expect(res.status).toEqual(200);
  });
});
