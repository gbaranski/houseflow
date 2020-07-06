import fetch from 'node-fetch';
import { username, password } from './globals';
import { AlarmRequestType, AlarmclockData } from '@gbaranski/types';
import { ALARMCLOCK_URL } from '../src/config';

describe('test set state on alarmclock', () => {
  it('attempting to set alarmclck state with invalid credentials', async () => {
    const res = await fetch('http://localhost:8000/alarmclock/switchState', {
      method: 'POST',
      headers: {
        username: 'randomUsername',
        password: 'randomPassword',
        state: '0',
      },
    });
    expect(res.status).toEqual(401);
  });
  it('attempting to set alarmclck state with valid credentials', async () => {
    const dataRes = await fetch(ALARMCLOCK_URL + AlarmRequestType.GET_DATA);
    const previousState: AlarmclockData = await dataRes.json();

    const res = await fetch('http://localhost:8000/alarmclock/switchState', {
      method: 'POST',
      headers: {
        username: username,
        password: password,
        state: String(!previousState.alarmState),
      },
    });
    expect(res.status).toEqual(200);

    // check if it really set to proper state
    const getDataRes = await fetch(ALARMCLOCK_URL + AlarmRequestType.GET_DATA);
    const alarmStateRes: AlarmclockData = await getDataRes.json();
    const expectedState = previousState ? 0 : 1;
    expect(alarmStateRes.alarmState).toEqual(expectedState);
  });
});
