import { Alarmclock } from '@gbaranski/types';

const remoteUrl = 'https://api.gbaranski.com';

function getHeaders() {
  const headers = new Headers();
  headers.append('username', localStorage.getItem('username') || '');
  headers.append('password', localStorage.getItem('password') || '');
  return headers;
}

export const login = async () => {
  const loginPageUrl = `${remoteUrl}/api/login`;
  const res = await fetch(loginPageUrl, {
    method: 'POST',
    headers: getHeaders(),
  });
  return res.status === 200;
};

export async function getDeviceStatus() {
  const alarmClockArrayUrl = `${remoteUrl}/api/getDeviceStatus`;
  const res = await fetch(alarmClockArrayUrl, {
    method: 'GET',
    headers: getHeaders(),
  });
  const json = await res.json();

  return await JSON.parse(json);
}
export async function getAlarmClockData(): Promise<Alarmclock.Data> {
  const alarmClockDataUrl = `${remoteUrl}/alarmclock/getData`;
  const res = await fetch(alarmClockDataUrl, {
    method: 'GET',
    headers: getHeaders(),
  });
  const json = await res.json();

  console.log(JSON.parse(await json));
  return await JSON.parse(json);
}

export async function sendTimeRequest(date: Date) {
  const formattedTime = `${date.getHours()}:${date.getMinutes()}`;
  const headers = getHeaders();
  headers.append('time', formattedTime);
  const res = await fetch(`${remoteUrl}/alarmclock/setTime`, {
    method: 'POST',
    headers,
  });
  return res.ok;
}

export async function switchAlarmState(state: boolean) {
  const headers = getHeaders();

  headers.append('state', String(+!state));
  const res = await fetch(`${remoteUrl}/alarmclock/switchState`, {
    method: 'POST',
    headers,
  });
  return res.ok;
}

export async function testSiren() {
  const headers = getHeaders();
  if (headers.get('username') !== 'gbaranski') {
    return false;
  } else {
    const res = await fetch(`${remoteUrl}/alarmclock/testSiren`, {
      method: 'POST',
      headers,
    });
    return res.ok;
  }
}

export async function getWatermixerData() {
  const alarmClockDataUrl = `${remoteUrl}/watermixer/getData`;
  const res = await fetch(alarmClockDataUrl, {
    method: 'GET',
    headers: getHeaders(),
  });
  const json = await res.json();

  console.log(JSON.parse(await json));
  return await JSON.parse(json);
}

export async function startMixing() {
  const res = await fetch(`${remoteUrl}/watermixer/startMixing`, {
    method: 'POST',
    headers: getHeaders(),
  });
  return res.ok;
}
