import {AlarmclockData, alarmclockSampleData} from "./alarmclock";
import {WatermixerData, watermixerSampleData} from "./watermixer";
import WebSocket from 'ws';
import { IncomingMessage } from 'http'

export interface Devices {
  alarmclock: {
    status: boolean;
    data: AlarmclockData;
    ws: WebSocket | undefined;
    req: IncomingMessage | undefined;

  }
  watermixer: {
    status: boolean;
    data: WatermixerData;
    ws: WebSocket | undefined;
    req: IncomingMessage | undefined;
  }
}

export interface DeviceStatus {
  alarmclock: boolean;
  watermixer: boolean;
  gate: boolean;
  garage: boolean;
}

export enum DeviceList {
  Alarmclock = 'Alarmclock',
  Watermixer = 'Watermixer',
  Gate = 'Gate',
  Garage = 'Garage',
}


export const devicesSample: Devices = {
  alarmclock: {
    status: false,
    data: alarmclockSampleData,
    ws: undefined,
    req: undefined,
  },
  watermixer: {
    status: false,
    data: watermixerSampleData,
    ws: undefined,
    req: undefined,
  }
}

export enum LocalIpAddress {
  Alarmclock = "192.168.1.110",
  Watermixer = "192.168.1.120",
}

export enum AlarmRequestType {
  GET_DATA = "/getESPData",
  GET_TEMP_ARRAY = "/getTempArray",
  GET_DEVICE_STATE = "/isDown",
  SET_TIME = "/setAlarm",
  SWITCH_STATE = "/setAlarmState",
  TEST_ALARM = "/testAlarm",
}
export enum WaterRequestType {
  GET_DATA = "/getESPData",
  START_MIXING = "/startMixing",
}

export interface TempHistory {
  unixTime: number;
  temperature: number;
}
export interface RequestHistory {
  user: string;
  requestPath: string;
  unixTime: number;
  ip: string;
  userAgent: string;
  country: string;
}

export enum OtherRequestsType {
  GET_DEVICES_STATUS = "/getDevicesStatus",
}

