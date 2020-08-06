import { AlarmclockData, DateTime, WatermixerData, State } from '.';

export interface RequestClient {
  deviceUid?: string;
  type: ClientRequests;
  data?: DateTime | State;
}
export enum ClientRequests {
  GET_DATA = 'GET_DATA',
  GET_DEVICES = 'GET_DEVICES',
  GET_DEVICES_STATUS = 'GET_DEVICES_STATUS',
}

export interface ResponseClient<
  T extends WatermixerData | AlarmclockData | undefined
> {
  ok: boolean;
  deviceUid: string;
  responseFor: ClientRequests;
  data: T;
}

// const Example: ClientRequest<DeviceType.ALARMCLOCK> = {
//   request: (type: RequestTypes, data?: DateTime | boolean) =>
//     console.log(type, data),
// };
