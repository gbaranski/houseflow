import React from 'react';

interface IWebsocketContext {
  websocket: WebSocket | undefined;
  setWebsocket: ((websocket: WebSocket) => any) | undefined;
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
