import WebSocket from 'ws';
import { DateTime } from './';
export interface AlarmclockData {
    alarmTime: DateTime;
    alarmState: boolean;
    sensor: {
        temperature: number;
        humidity: number;
        heatIndex: number;
    };
}
export declare const alarmclockSampleData: AlarmclockData;
export interface Alarmclock {
    alarmclock: {
        status: boolean;
        data: AlarmclockData;
        ws: WebSocket | undefined;
    };
}
//# sourceMappingURL=alarmclock.d.ts.map