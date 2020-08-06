import { WSS_URL } from '../config';

export const beginWebSocketConnection = (
  token: string,
  setWebsocket: ((websocket: WebSocket) => any) | undefined,
) => {
  if (!setWebsocket) throw new Error('Set websocket is not defined');
  const ws = new WebSocket(WSS_URL, token);

  setWebsocket(ws);
  setupEventListeners(ws);
};

const setupEventListeners = (ws: WebSocket) => {
  ws.addEventListener('open', (event) => {
    ws.addEventListener('message', (wsResponse) =>
      console.log(JSON.parse(wsResponse.data)),
    );
    ws.addEventListener('close', (event) => {
      console.log('Closed connection');
    });
  });
};
