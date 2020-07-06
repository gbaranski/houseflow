/* eslint-disable @typescript-eslint/explicit-function-return-type */
import fetch from 'node-fetch';
import { username, password } from './globals';
import { AlarmclockData, AlarmRequestType } from '@gbaranski/types';
import { ALARMCLOCK_URL } from '../src/config';

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
    const resJson = await JSON.parse(await res.json());
    console.dir(await resJson);

    const entries = Object.entries(resJson);
    const realTypeLength = 7; // sorry for hardcoded, but i couldn't get length of interface
    expect(entries.length).toEqual(realTypeLength);

    expect(res.status).toEqual(200);
  });

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
    expect(res.status).toEqual(200);
  });
  it('attempting to set alarmclck time with invalid credentials', async () => {
    const res = await fetch('http://localhost:8000/alarmclock/setTime', {
      method: 'POST',
      headers: {
        username: 'randomUsername',
        password: 'randomPassword',
        time: '12:30',
      },
    });
    expect(res.status).toEqual(401);
  });
  it('attempting to set alarmclck time with valid credentials', async () => {
    const time = '12:30';
    const res = await fetch('http://localhost:8000/alarmclock/setTime', {
      method: 'POST',
      headers: {
        username: username,
        password: password,
        time,
      },
    });
    expect(res.status).toEqual(200);

    // check if it really set to proper time
    const getDataRes = await fetch(ALARMCLOCK_URL + AlarmRequestType.GET_DATA);
    const alarmTimeRes: AlarmclockData = await getDataRes.json();
    expect(alarmTimeRes.alarmTime).toEqual(time);
  });
});
