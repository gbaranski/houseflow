import { WSS_URL } from '../config';
import { Device, Client, AnyDeviceData } from '@gbaranski/types';
import { toast } from 'react-toastify';

export const beginWebsocketConnection = async (
  token: string,
): Promise<WebSocket> => {
  console.log('Starting websocket service');
  return new WebSocket(WSS_URL, token);
};

export const setupWebsocketHandlers = async (
  ws: WebSocket,
  devices: Device.ActiveDevice<AnyDeviceData>[],
  setDevices: (devices: Device.ActiveDevice<AnyDeviceData>[]) => any,
  firstDataArrived: boolean,
  setFirstDataArrived: (state: boolean) => any,
  onConnectionClose: () => any,
) => {
  ws.addEventListener('message', (wsResponse) => {
    handleMessage(
      wsResponse.data,
      devices,
      setDevices,
      firstDataArrived,
      setFirstDataArrived,
    );
  });
};

const handleMessage = (
  message: string,
  devices: Device.ActiveDevice<AnyDeviceData>[],
  setDevices: (devices: Device.ActiveDevice<AnyDeviceData>[]) => any,
  firstDataArrived: boolean,
  setFirstDataArrived: (state: boolean) => any,
) => {
  const response = JSON.parse(message) as Client.Response;
  if (!response) throw new Error('Websocket response is not okay!');
  if (response.requestType === 'DATA') {
    console.log('Received new data', response.data);
    setDevices(response.data as Device.ActiveDevice<AnyDeviceData>[]);
    if (!firstDataArrived) {
      setFirstDataArrived(true);
    }
  } else if (response.requestType === 'DEVICES') {
    console.log('Recieved devices', response.data);
  }
};

export const preWebsocketMessage = (ws: WebSocket) => {
  try {
    if (!ws) throw new Error('Websocket is not defined');
    if (!ws.OPEN) throw new Error('Websocket connection is not opened');
  } catch (e) {
    toast.error('Failed! ❌', {
      autoClose: 1000,
      progressStyle: {
        background: 'red',
      },
      bodyStyle: {
        color: 'gray',
      },
    });
    throw e;
  }
  toast('Sending request! ✅', {
    autoClose: 1000,
    progressStyle: {
      background: 'green',
    },
    bodyStyle: {
      color: 'gray',
    },
  });
};
