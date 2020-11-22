import { GeoPoint } from '.';

export namespace Device {
  export enum Action {
    OpenGate = 'open_gate',
    OpenGarage = 'open_garage',
    SwitchLights = 'switch_lights',
    MixWater = 'mix_water',
  }

  export enum DeviceType {
    WATERMIXER = 'WATERMIXER',
    GATE = 'GATE',
    GARAGE = 'GARAGE',
    LIGHT = 'LIGHT',
  }

  export interface Request {
    correlationData: string;
    data?: unknown | undefined;
  }

  export interface Response {
    correlationData: string;
    data?: unknown | undefined;
  }

  export interface FirebaseDevice<T = undefined> {
    uid: string;
    secret?: string;
    type: DeviceType;
    geoPoint: GeoPoint;
    actions: Action[];

    status: boolean;
    data: T;
    ip: string;
  }
}
