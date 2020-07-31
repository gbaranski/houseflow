/// <reference types="node" />
import { AlarmclockData } from "./alarmclock";
import { WatermixerData } from "./watermixer";
import WebSocket from 'ws';
import { IncomingMessage } from 'http';
export interface Devices {
    alarmclock: {
        status: boolean;
        data: AlarmclockData;
        ws: WebSocket | undefined;
        req: IncomingMessage | undefined;
    };
    watermixer: {
        status: boolean;
        data: WatermixerData;
        ws: WebSocket | undefined;
        req: IncomingMessage | undefined;
    };
}
export interface DeviceStatus {
    alarmclock: boolean;
    watermixer: boolean;
    gate: boolean;
    garage: boolean;
}
export declare enum DeviceList {
    Alarmclock = "Alarmclock",
    Watermixer = "Watermixer",
    Gate = "Gate",
    Garage = "Garage"
}
export declare const devicesSample: Devices;
export declare enum LocalIpAddress {
    Alarmclock = "192.168.1.110",
    Watermixer = "192.168.1.120"
}
export declare enum AlarmRequestType {
    GET_DATA = "/getESPData",
    GET_TEMP_ARRAY = "/getTempArray",
    GET_DEVICE_STATE = "/isDown",
    SET_TIME = "/setAlarm",
    SWITCH_STATE = "/setAlarmState",
    TEST_ALARM = "/testAlarm"
}
export declare enum WaterRequestType {
    GET_DATA = "/getESPData",
    START_MIXING = "/startMixing"
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
export declare enum OtherRequestsType {
    GET_DEVICES_STATUS = "/getDevicesStatus"
}
//# sourceMappingURL=other.d.ts.map