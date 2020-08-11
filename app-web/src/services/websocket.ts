import { WSS_URL } from '../config';

export const beginWebSocketConnection = (token: string) => {
  const websocket = new WebSocket(WSS_URL, token);
  websocket.addEventListener('open', (event) => {
    console.log('Connected');
    websocket.addEventListener('close', (event) => {
      console.log('Closed connection');
    });
  });
};
