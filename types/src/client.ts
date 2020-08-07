import {
  AlarmclockData,
  DateTime,
  WatermixerData,
  State,
  DeviceType,
  CurrentDevice,
  AnyDeviceData,
} from '.';

export interface RequestClient {
  deviceUid?: string;
  type: ClientRequests;
  deviceType?: DeviceType;
  data?: DateTime | State | ClientCurrentDevice<DeviceType>[];
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

export interface ClientCurrentDevice<T extends DeviceType> {
  type: T;
  secret: string;
  uid: string;
  data?: TDeviceData<T>;
  status?: State;
}

export interface CurrentDeviceData {
  type: DeviceType;
  uid: string;
  data: AnyDeviceData;
  status: State;
}

export type TDeviceDataArgs<T extends AnyDeviceData | undefined> = {
  devicesData: T;
  setDevicesData: ((data: T) => any) | undefined;
};

type TDeviceData<
  T extends DeviceType | undefined
> = T extends DeviceType.ALARMCLOCK
  ? TDeviceDataArgs<AlarmclockData>
  : T extends DeviceType.WATERMIXER
  ? TDeviceDataArgs<WatermixerData>
  : undefined;

// const Example: ClientRequest<DeviceType.ALARMCLOCK> = {
//   request: (type: RequestTypes, data?: DateTime | boolean) =>
//     console.log(type, data),
// };
