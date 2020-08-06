import React from 'react';

export type TSetWebsocket = ((websocket: WebSocket) => any) | undefined;

interface IWebsocketContext {
  websocket: WebSocket | undefined;
  setWebsocket: TSetWebsocket;
}

export const WebsocketContext = React.createContext<IWebsocketContext>({
  websocket: undefined,
  setWebsocket: undefined,
});

interface WebsocketProviderProps {
  children: React.ReactNode;
}

export const WebsocketProvider = ({ children }: WebsocketProviderProps) => {
  const [websocket, setWebsocket] = React.useState<WebSocket | undefined>();
  return (
    <WebsocketContext.Provider
      value={{
        websocket,
        setWebsocket,
      }}
    >
      {children}
    </WebsocketContext.Provider>
  );
};
