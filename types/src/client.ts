import { AlarmclockData, DateTime, WatermixerData, State } from '.';
import { DeviceType, CurrentDevice } from './device';

export interface RequestClient {
  deviceUid?: string;
  type: ClientRequests;
  deviceType?: DeviceType;
  data?: DateTime | State;
}
export enum ClientRequests {
  GET_DATA = 'GET_DATA',
  GET_DEVICES = 'GET_DEVICES',
  GET_DEVICES_STATUS = 'GET_DEVICES_STATUS',
}

export interface ResponseClient<
  T extends WatermixerData | AlarmclockData | CurrentDevice[] | undefined
> {
  ok: boolean;
  deviceUid?: string;
  deviceType?: DeviceType;
  responseFor: ClientRequests;
  data: T;
}

// const Example: ClientRequest<DeviceType.ALARMCLOCK> = {
//   request: (type: RequestTypes, data?: DateTime | boolean) =>
//     console.log(type, data),
// };
