import { useState } from 'react';
import { Client, Device, AnyDeviceData } from '@gbaranski/types';

export default () => {
  const [websocket, setWebsocket] = useState<WebSocket | undefined>(undefined);

  const sendMessage = (message: Client.Request): boolean => {
    if (!websocket || !websocket.OPEN) return false;
    websocket.send(JSON.stringify(message));
    return true;
  };

  const beginWebsocket = (
    setActiveDevies: (devices: Device.ActiveDevice<AnyDeviceData>[]) => any,
  ) => {
    const { WSS_URL } = process.env;
    if (!WSS_URL) throw new Error('WSS_URL is not defined in .env');
    const ws = new WebSocket(WSS_URL);
    setWebsocket(ws);
    ws.onopen = () => {
      console.log('Connection opened');
      ws.onmessage = (message) => {
        const parsedMessage = JSON.parse(message.data) as Client.Response;
        if (!parsedMessage) throw new Error('Invalid websocket response');
        if (parsedMessage.requestType === 'DATA') {
          const activeDevices = parsedMessage.data as Device.ActiveDevice<AnyDeviceData>[];
          setActiveDevies(activeDevices);
        }
      };
    };
  };

  return {
    websocket,
    beginWebsocket,
    sendMessage,
  };
};
