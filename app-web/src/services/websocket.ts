import { WSS_URL } from '../config';
import { ResponseClient, CurrentDevice, DeviceType } from '@gbaranski/types';
import { TSetWebsocket } from '../providers/websocketProvider';
import { ClientCurrentDevice } from '../providers/deviceDataProvider';

export const beginWebSocketConnection = (
  token: string,
  setWebsocket: TSetWebsocket,
  devices: ClientCurrentDevice<DeviceType>[],
  setDevices: (devices: ClientCurrentDevice<DeviceType>[]) => any,
) => {
  if (!setWebsocket) throw new Error('Set websocket is not defined');
  const ws = new WebSocket(WSS_URL, token);

  setWebsocket(ws);
  ws.addEventListener('open', (event) => {
    ws.addEventListener('message', (wsResponse) => {
      handleMessage(wsResponse.data, devices, setDevices);
    });
    ws.addEventListener('close', (event) => {
      console.log('Closed connection');
    });
  });
};

const handleMessage = (
  message: string,
  devices: ClientCurrentDevice<DeviceType>[],
  setDevices: (devices: ClientCurrentDevice<DeviceType>[]) => any,
) => {
  const response = JSON.parse(message) as ResponseClient<undefined>;
  if (!response.ok) throw new Error('Websocket response is not okay!');
  if (response.responseFor == 'GET_DATA') {
    console.log('Received new data from server!', response);
  }
};
