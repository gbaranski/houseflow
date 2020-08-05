import { DeviceType } from './other';
import { RequestAlarmclock, RequestWatermixer } from '.';

export interface CurrentUser {
  fullAccess: string[];
  name: string;
  uid: string;
}

export interface ClientRequest<T extends DeviceType> {
  device?: T;
  request: T extends DeviceType.ALARMCLOCK
    ? RequestAlarmclock
    : T extends DeviceType.WATERMIXER
    ? RequestWatermixer
    : undefined;
}

// const Example: ClientRequest<DeviceType.ALARMCLOCK> = {
//   request: (type: RequestTypes, data?: DateTime | boolean) =>
//     console.log(type, data),
// };
