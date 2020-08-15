import { Client } from '@gbaranski/types';

const WSS_URL =
  process.env.NODE_ENV === 'development' ? 'ws://localhost:8001' : 'wss://wsc.gbaranski.com:443';

console.log({ processenv: process.env.NODE_ENV });
let websocket: WebSocket | undefined;

export const connectWebsocket = (token: string) => {
  websocket = new WebSocket(WSS_URL, token);
};

export const setupOnOpenListeners = (onMessage: (message: MessageEvent) => any) => {
  if (!websocket) throw new Error('Websocket is not defined');
  if (websocket.OPEN) {
    websocket.onmessage = onMessage;
    return;
  }
  websocket.onopen = () => {
    if (!websocket) throw new Error('Websocket is not defined');
    websocket.onmessage = onMessage;
  };
};

export const getWebsocket = (): WebSocket | undefined => {
  return websocket;
};

export const sendCurrentConnectionsRequest = () => {
  if (!websocket) throw new Error('Websocket is not defined');
  const req: Client.Request = {
    requestType: 'CONNECTIONS',
  };
  websocket.send(JSON.stringify(req));
};
