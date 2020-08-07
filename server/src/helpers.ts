import WebSocket from 'ws';

export const validateDeviceMessage = (message: WebSocket.Data): void => {
  if (
    message instanceof Buffer ||
    message instanceof ArrayBuffer ||
    message instanceof Array
  )
    throw new Error('Cannot handle Buffer type');
  const parsedResponse = JSON.parse(message) as { ok: boolean };
  if (!parsedResponse.ok) throw new Error('Response is not okay');
};

export const validateClientMessage = (message: WebSocket.Data): void => {
  if (
    message instanceof Buffer ||
    message instanceof ArrayBuffer ||
    message instanceof Array
  )
    throw new Error('Cannot handle Buffer type');
};
