/* eslint-disable @typescript-eslint/explicit-function-return-type */
import fetch from 'node-fetch';
import { username, password } from './globals';

describe('Alarmclock endpoints', () => {
  it('attempting to get alarmclock data with no credentials', async () => {
    const res = await fetch('http://localhost:8000/alarmclock/getData', {
      method: 'GET',
    });
    expect(res.status).toEqual(401);
  });
  it('attempting to get alarmclock data with invalid credentials', async () => {
    const res = await fetch('http://localhost:8000/alarmclock/getData', {
      method: 'GET',
      headers: {
        username: 'randomUsername',
        password: 'randomPassword',
      },
    });
    expect(res.status).toEqual(401);
  });
  it('attempting to get alarmclock data with valid credentials', async () => {
    const res = await fetch('http://localhost:8000/alarmclock/getData', {
      method: 'GET',
      headers: {
        username: username,
        password: password,
      },
    });
    expect(res.status).toEqual(200);
  });
});
