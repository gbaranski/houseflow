import WebSocket from "ws";
import { Device, Watermixer } from "@gbaranski/types";
import { remainingSeconds, isTimerOn } from "./relay";

export const sendData = (ws: WebSocket) => {

    const data: Watermixer.Data = {
        remainingSeconds: remainingSeconds,
        isTimerOn: isTimerOn,
    }
    const response: Device.ResponseDevice<Watermixer.Data> = {
        responseFor: 'GET_DATA',
        ok: true,
        data: data,
        // @ts-ignore
        rand: Math.random(),
    };
    ws.send(JSON.stringify(response));
    console.log(ws.readyState);
}
