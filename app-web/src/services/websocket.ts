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
  console.log({ socket });
};

export const setupOnOpenListeners = (onDeviceData: (message: string) => any) => {
  if (!socket) throw new Error('Websocket is not defined');
  socket.on('open', () => {
    if (!socket) throw new Error('Websocket is not defined');

    socket.removeEventListener('device_data');
    socket.on('device_data', onDeviceData);
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
