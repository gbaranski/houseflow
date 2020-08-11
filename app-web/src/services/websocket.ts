import { WSS_URL } from '../config';
import { Device, Client, AnyDeviceData } from '@gbaranski/types';

export const beginWebsocketConnection = async (
  token: string,
): Promise<WebSocket> => {
  console.log('Starting websocket service');
  return new WebSocket(WSS_URL, token);
};

export const setupWebsocketHandlers = async (
  ws: WebSocket,
  setDevices: (devices: Device.ActiveDevice<AnyDeviceData>[]) => any,
  onConnectionClose: () => any,
) => {
  ws.addEventListener('message', (wsResponse) => {
    handleMessage(wsResponse.data, setDevices);
  });
};

const handleMessage = (
  message: string,
  setDevices: (devices: Device.ActiveDevice<AnyDeviceData>[]) => any,
) => {
  const response = JSON.parse(message) as Client.Response;
  if (!response) throw new Error('Websocket response is not okay!');
  if (response.requestType === 'DATA') {
    console.log('Received new data', response.data);
  } else if (response.requestType === 'DEVICES') {
    console.log('Recieved devices', response.data);
  }
};
