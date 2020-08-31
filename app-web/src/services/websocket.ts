const WSS_URL =
  process.env.NODE_ENV === 'development' ? 'ws://localhost:8001' : 'wss://api.gbaranski.com:443/wsc';

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
  console.log("Not implemented");
  // if (!websocket) throw new Error('Websocket is not defined');
  // const req: Device.RequestDevice = {
  //   requestType: 'CONNECTIONS',
  // };
  // websocket.send(JSON.stringify(req));
};
