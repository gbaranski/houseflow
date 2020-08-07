import { WSS_URL } from '../config';
import {
  ResponseClient,
  CurrentDevice,
  DeviceType,
  RequestClient,
  ClientRequests,
  ClientCurrentDevice,
} from '@gbaranski/types';
import { TSetWebsocket } from '../providers/websocketProvider';

export const beginWebsocketConnection = async (
  token: string,
): Promise<WebSocket> => {
  console.log('Starting websocket service');
  return new WebSocket(WSS_URL, token);
};

export const setupWebsocketHandlers = async (
  ws: WebSocket,
  setDevices: (devices: ClientCurrentDevice<DeviceType>[]) => any,
  onConnectionClose: () => any,
) => {
  ws.addEventListener('message', async (wsResponse) => {
    handleMessage(wsResponse.data, setDevices);
  });
};

export const getDevicesStatus = (
  ws: WebSocket,
  resolvedDevices: ClientCurrentDevice<DeviceType>[],
  callback?: () => any,
) => {
  console.log('Retreiving device status');
  const getDevicesStatusReq: RequestClient = {
    type: ClientRequests.GET_DEVICES_STATUS,
    data: resolvedDevices,
  };
  ws.send(JSON.stringify(getDevicesStatusReq));
  if (callback) callback();
};

const handleMessage = (
  message: string,
  setDevices: (devices: ClientCurrentDevice<DeviceType>[]) => any,
) => {
  const response = JSON.parse(message) as ResponseClient<undefined>;
  if (!response.ok) throw new Error('Websocket response is not okay!');
  console.log(response);
  if (response.responseFor === ClientRequests.GET_DEVICES) {
    // const devices: ResponseClient<
    //   ClientCurrentDevice<DeviceType>[]
    // > = console.log('Received new data from server!', response);
  } else if (response.responseFor === ClientRequests.GET_DATA) {
    if (!response.deviceType) throw new Error('Device type is not defined');
    // const deviceType: DeviceType =
    //   DeviceType[response.deviceType as keyof typeof DeviceType];
    // const deviceData = (response as ClientCurrentDevice<typeof deviceType>).data.
    // console.log("Received device data from server");
  }
};
