import express from 'express';
import fetch, { Headers } from 'node-fetch';
import { AlarmRequestType, TempArray, AlarmclockData } from '@gbaranski/types';

import { isAuthenticated } from '../auth';
import { sendMessage } from '../firebase';
import { getIp } from '../helpers';

const URL = 'http://192.168.1.110';
const HOURS_IN_DAY = 24;
const SECONDS_IN_HOUR = 3600;

let secondsPassed = SECONDS_IN_HOUR;

const temperaturesArr: TempArray[] = new Array(HOURS_IN_DAY);

let data: AlarmclockData;

let isProcessing: boolean = false;

export async function AlarmclockHandleRequest(
  req: express.Request,
  res: express.Response,
  requestType: AlarmRequestType,
) {
  if (!isAuthenticated(req.header('username') || '', req.header('password') || '')) {
    console.log(`${getIp(req)} with ${req.get('user-agent')} on ${requestType} not authenticated`);
    res.status(401).end();
    return;
  }
  console.log(`${getIp(req)} with ${req.get('user-agent')} on ${requestType} authenticated`);
  const headers = new Headers();
  switch (requestType) {
    case AlarmRequestType.GET_DATA:
      res.json(JSON.stringify(data));
      break;
    case AlarmRequestType.GET_TEMP_ARRAY:
      res.json(JSON.stringify(temperaturesArr));
      break;
    case AlarmRequestType.TEST_ALARM:
      await res.status(await fetchURL(AlarmRequestType.TEST_ALARM, headers)).end();
      if (req.header('username') !== 'gbaranski') {
        sendMessage(req.header('username') || '', `alarmclock${requestType}`);
      }
      break;
    case AlarmRequestType.SET_TIME:
      headers.append('time', req.header('time') || '');
      await res.status(await fetchURL(AlarmRequestType.SET_TIME, headers)).end();
      if (req.header('username') !== 'gbaranski') {
        sendMessage(req.header('username') || '', `alarmclock${requestType}`);
      }
      break;
    case AlarmRequestType.SWITCH_STATE:
      headers.append('state', req.header('state') || '');
      await res.status(await fetchURL(AlarmRequestType.SWITCH_STATE, headers)).end();
      if (req.header('username') !== 'gbaranski') {
        sendMessage(req.header('username') || '', `alarmclock${requestType}`);
      }
      break;
    default:
      res.status(500).end();
      break;
  }
}

async function fetchURL(path: string, headers: Headers): Promise<number> {
  isProcessing = true;
  let statusCode = 0;
  await fetch(URL + path, {
    method: 'POST',
    headers,
  })
    .then(_data => {
      console.log('Success:', _data.status);
      statusCode = _data.status;
    })
    .catch(error => {
      console.error('Error:', error);
      statusCode = 503;
    });
  isProcessing = false;
  return statusCode;
}

export async function AlarmclockFetchEspDataInterval(setStatus: (state: boolean) => void) {
  secondsPassed += 1;
  if (isProcessing) {
    console.log('Connection overloaded');
    temperaturesArr.shift();
    temperaturesArr.push({ unixTime: new Date().getTime(), temp: data.temperature });
    return;
  }
  isProcessing = true;
  fetch(URL + AlarmRequestType.GET_DATA)
    .then(res => res.json())
    .then((_data: AlarmclockData) => {
      data = _data;
      secondsPassed += 1;
      if (secondsPassed >= SECONDS_IN_HOUR) {
        temperaturesArr.shift();
        temperaturesArr.push({ unixTime: new Date().getTime(), temp: _data.temperature });
      }
      setStatus(true);
      console.log('Fetched alarmclock data');
    })
    .catch(error => {
      setStatus(false);
      console.log('Error while fetching alarmclock', error);
    })
    .finally(() => {
      isProcessing = false;
    });
}
