/* eslint-disable @typescript-eslint/explicit-function-return-type */
import fetch from 'node-fetch';
import { username, password } from './globals';

describe('Login endpoints', () => {
  it('attempting to login with no credentials', async () => {
    const res = await fetch('http://localhost:8000/api/login', {
      method: 'POST',
    });
    expect(res.status).toEqual(401);
  });
  it('attempting to login with invalid credentials', async () => {
    const res = await fetch('http://localhost:8000/api/login', {
      method: 'POST',
      headers: {
        username: 'randomUsername',
        password: 'randomPassword',
      },
    });
    expect(res.status).toEqual(401);
  });
  it('attempting to login with valid credentials', async () => {
    const res = await fetch('http://localhost:8000/api/login', {
      method: 'POST',
      headers: {
        username: username,
        password: password,
      },
    });
    expect(res.status).toEqual(200);
  });
});
