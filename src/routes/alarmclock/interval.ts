import fetch from 'node-fetch';
import { AlarmRequestType, AlarmclockData, TempArray } from '@gbaranski/types';
import { getProcessing, setDeviceStatus, getDeviceStatus } from '../globals';
import { setProcessingAlarmclock } from '.';
import { ALARMCLOCK_URL } from '../../config';

const HOURS_IN_DAY = 24;
const SECONDS_IN_HOUR = 3600;

let secondsPassed = SECONDS_IN_HOUR;

const temperaturesArr: TempArray[] = new Array(HOURS_IN_DAY).fill(undefined);
temperaturesArr.forEach((elem, index): void => {
  temperaturesArr[index] = {
    unixTime: new Date(
      new Date().getTime() - 60 * 60 * (index + 1) * 1000,
    ).getTime(),
    temp: 0,
  };
});

let data: AlarmclockData;

export async function alarmclockInterval(): Promise<void> {
  secondsPassed += 1;
  if (getProcessing().alarmclock) {
    console.log('Connection overloaded at alarmclock');
    temperaturesArr.shift();
    temperaturesArr.push({
      unixTime: new Date().getTime(),
      temp: data.temperature,
    });
    return;
  }
  setProcessingAlarmclock(true);
  fetch(ALARMCLOCK_URL + AlarmRequestType.GET_DATA)
    .then((res): Promise<AlarmclockData> => res.json())
    .then((_data: AlarmclockData): void => {
      data = {
        ..._data,
      };
      if (secondsPassed >= SECONDS_IN_HOUR) {
        temperaturesArr.shift();
        temperaturesArr.push({
          unixTime: new Date().getTime(),
          temp: _data.temperature,
        });
        secondsPassed = 0;
      }
      setDeviceStatus({
        ...getDeviceStatus(),
        alarmclock: true,
      });
    })
    .catch((): void => {
      console.log('Error while fetching alarmclock!');
    })
    .finally((): void => {
      setProcessingAlarmclock(false);
    });
}

export function getData(): AlarmclockData {
  return data;
}

export function getTempArray(): TempArray[] {
  return temperaturesArr;
}
