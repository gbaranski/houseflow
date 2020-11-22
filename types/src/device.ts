import { GeoPoint } from '.';

export namespace Device {
  export enum Action {
    OpenGate = 'action_open_gate',
    OpenGarage = 'action_open_garage',
    SwitchLights = 'action_switch_lights',
    MixWater = 'action_mix_water',
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
