import WebSocket from 'ws';
export interface Watermixer {
    status: boolean;
    data: WatermixerData;
    ws: WebSocket | undefined;
}
export interface WatermixerData {
    remainingSeconds: number;
    isTimerOn: boolean;
}
//# sourceMappingURL=watermixer.d.ts.map