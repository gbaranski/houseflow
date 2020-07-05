import { AlarmRequestType, AlarmclockData, TempArray } from '@gbaranski/types';
import { ALARMCLOCK_URL } from '@/config';

const HOURS_IN_DAY = 24;
const SECONDS_IN_HOUR = 3600;

let secondsPassed = SECONDS_IN_HOUR;

const temperaturesArr: TempArray[] = new Array(HOURS_IN_DAY).fill(undefined);
temperaturesArr.forEach((elem, index) => {
  temperaturesArr[index] = {
    unixTime: new Date(new Date().getTime() - 60 * 60 * (index + 1) * 1000).getTime(),
    temp: 0,
  };
});

let data: AlarmclockData;

let isProcessing: boolean = false;

export async function AlarmclockInterval(setStatus: (state: boolean) => void) {
  secondsPassed += 1;
  if (isProcessing) {
    console.log('Connection overloaded');
    temperaturesArr.shift();
    temperaturesArr.push({ unixTime: new Date().getTime(), temp: data.temperature });
    return;
  }
  isProcessing = true;
  fetch(ALARMCLOCK_URL + AlarmRequestType.GET_DATA)
    .then(res => res.json())
    .then((_data: AlarmclockData) => {
      data = _data;
      if (secondsPassed >= SECONDS_IN_HOUR) {
        temperaturesArr.shift();
        temperaturesArr.push({ unixTime: new Date().getTime(), temp: _data.temperature });
        secondsPassed = 0;
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

export function getData(): AlarmclockData {
  return data;
}

export function getTempArray(): TempArray[] {
  return temperaturesArr;
}
