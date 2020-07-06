/* eslint-disable @typescript-eslint/explicit-function-return-type */
import fetch from 'node-fetch';
import { username, password } from './globals';

describe('testing watermixer mixing', () => {
  it('attempting to start watermixer mixing with no credentials', async () => {
    const res = await fetch('http://localhost:8000/watermixer/startMixing', {
      method: 'GET',
    });
    expect(res.status).toEqual(401);
  });
  it('attempting to start watermixer mixing with invalid credentials', async () => {
    const res = await fetch('http://localhost:8000/watermixer/startMixing', {
      method: 'GET',
      headers: {
        username: 'randomUsername',
        password: 'randomPassword',
      },
    });
    expect(res.status).toEqual(401);
  });
  //   it('attempting to start watermixer mixing with valid credentials', async () => {
  //     const res = await fetch('http://localhost:8000/watermixer/getData', {
  //       method: 'GET',
  //       headers: {
  //         username: username,
  //         password: password,
  //       },
  //     });
  //     const resJson = await JSON.parse(await res.json());
  //     console.dir(await resJson);

  //     const entries = Object.entries(resJson);
  //     const realTypeLength = 2; // sorry for hardcoded, but i couldn't get length of interface
  //     expect(entries.length).toEqual(realTypeLength);

  //     expect(res.status).toEqual(200);
  //   });
});
