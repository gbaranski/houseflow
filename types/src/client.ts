import { DateTime, State, AnyDeviceData } from '.';
import { Device } from './device';

export namespace Client {
  export type RequestType = 'TEST' & Device.RequestType;

  export interface Request {
    requestType: RequestType;
    deviceUid?: string;
    deviceType?: string;
    data?: DateTime | State;
  }

  export type ResponseType = 'DATA' | 'DEVICES' | 'DEVICES_STATUS';

  export interface Response {
    requestType: ResponseType;
    data?: ActiveDevice;
  }

  export interface FirebaseDevice {
    type: Device.DeviceType;
    uid: string;
    secret: string;
  }

  export interface ActiveDevice extends FirebaseDevice {
    data: AnyDeviceData;
    status: State;
  }
}
