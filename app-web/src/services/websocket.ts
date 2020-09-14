import io from 'socket.io-client';

const SOCKET_URL =
  process.env.NODE_ENV === 'development'
    ? 'http://localhost:8001'
    : 'https://api.gbaranski.com:443/wsc';

console.log({ processenv: process.env.NODE_ENV });

export const connectWebsocket = (token: string) => {
  return new Promise<SocketIOClient.Socket>((resolve, reject) => {
    const socket = io(SOCKET_URL, {
      query: {
        token,
      },
    });
    socket.on('connect', () => {
      console.log('Connected to socket');
      resolve(socket);
    });
    socket.on('connet_error', (error: Error) => reject(error));
  });
};
