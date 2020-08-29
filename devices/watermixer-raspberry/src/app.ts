import WebSocket from "ws"
import { Device } from "@gbaranski/types";
import { startMixing } from "@/services/relay";
import { sendData } from "./services/websocket";

let isAlive = false;

export const onConnection = (ws: WebSocket): void => {
    console.log('Connection opened')
    setupIntervals(ws);
    ws.on('close', (code, reason) => console.log("Closed", { code, reason }))
    ws.on('message', (data) => handleMessage(data));
    ws.on('error', console.log)
    ws.on('unexpected-response', console.log)
    ws.on('pong', () => {
        console.log("Connection alive!");
        isAlive = true;
    });
}

const setupIntervals = (ws: WebSocket) => {
    isAlive = true;

    // pingInterval
    setInterval(() => {
        if (!isAlive) throw new Error("Ping not recieved");
        isAlive = false;
        ws.ping();
    }, 5000)

    setInterval(() => sendData(ws), 500)
}



export const handleMessage = (message: WebSocket.Data): void => {
    console.log("Received new message", message);
    try {
        if (message instanceof Buffer) throw new Error("Message cannot be instance of an buffer");
        if (message instanceof ArrayBuffer) throw new Error("Message cannot be instance of an arraybuffer");
        if (message instanceof Array) throw new Error("Message cannot be instance of an array");

        const request = JSON.parse(message) as Device.RequestDevice;
        if (!request.type) throw new Error("Type does not exists");

        switch (request.type) {
            case 'START_MIXING':
                startMixing();
                break;
            default:
                throw new Error("Unable to handle");
                break;
        }


    } catch (e) {
        console.log(`${e.message} error when handling message`);
    }
} 