import fetch from 'node-fetch';
import { username, password } from './globals';

describe('test set time on alarmclock', () => {
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
    expect(res.status).toEqual(201);

    // check if it really set to proper time
    // const getDataRes = await fetch(ALARMCLOCK_URL + AlarmRequestType.GET_DATA);
    // const alarmTimeRes: AlarmclockData = await getDataRes.json();
    // expect(alarmTimeRes.alarmTime).toEqual(time);
  });
});
