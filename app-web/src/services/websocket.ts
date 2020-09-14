import { mdiPowerSocketIt } from '@mdi/js';
import io from 'socket.io-client';

const SOCKET_URL =
  process.env.NODE_ENV === 'development'
    ? 'http://localhost:8001'
    : 'https://api.gbaranski.com:443/wsc';

console.log({ processenv: process.env.NODE_ENV });
let socket: SocketIOClient.Socket | undefined;

export const connectWebsocket = (token: string) => {
  socket = io(SOCKET_URL, {
    query: {
      token,
    },
  });
  setupOnOpenListeners();
  console.log({ socket });
};

export const setupOnOpenListeners = () => {
  console.log('SetupOnOpenListeners');
  if (!socket) throw new Error('Websocket is not defined');
  socket.on('connect', () => {
    console.log('Socket opened');
    if (!socket) throw new Error('Websocket is not defined');
    socket.on('device_data', (data: any) => console.log(data));
  });
};

export const getWebsocket = (): SocketIOClient.Socket | undefined => {
  return socket;
};

export const sendCurrentConnectionsRequest = () => {
  console.log('Not implemented');
  // if (!websocket) throw new Error('Websocket is not defined');
  // const req: Device.RequestDevice = {
  //   requestType: 'CONNECTIONS',
  // };
  // websocket.send(JSON.stringify(req));
};
