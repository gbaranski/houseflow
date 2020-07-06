/* eslint-disable @typescript-eslint/explicit-function-return-type */
import fetch from 'node-fetch';
import { username, password } from './globals';

describe('testing retreiving device status', () => {
  it('attempting to get device status with no credentials', async () => {
    const res = await fetch('http://localhost:8000/api/getDeviceStatus', {
      method: 'GET',
    });
    expect(res.status).toEqual(401);
  });
  it('attempting to get device status with invalid credentials', async () => {
    const res = await fetch('http://localhost:8000/api/getDeviceStatus', {
      method: 'GET',
      headers: {
        username: 'randomUsername',
        password: 'randomPassword',
      },
    });
    expect(res.status).toEqual(401);
  });
  it('attempting to get device status with valid credentials', async () => {
    const res = await fetch('http://localhost:8000/api/getDeviceStatus', {
      method: 'GET',
      headers: {
        username: username,
        password: password,
      },
    });
    expect(res.status).toEqual(200);
  });
});
