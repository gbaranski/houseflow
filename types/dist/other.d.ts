import { AlarmclockData, WatermixerData } from './';
export interface DateTime {
    hour: number;
    minute: number;
    second: number;
}
export declare enum RequestTypes {
    GET_DATA = 0,
    SET_STATE = 1,
    SET_TIME = 2,
    START_MIXING = 3
}
export declare type State = boolean;
export declare type RequestDevice = ((type: RequestTypes.SET_TIME, data: DateTime) => any) & ((type: RequestTypes.SET_STATE, data: boolean) => any);
export interface ResponseDevice {
    ok: boolean;
    responseFor: RequestTypes;
    data?: AlarmclockData | WatermixerData | 'OK';
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
//# sourceMappingURL=other.d.ts.map